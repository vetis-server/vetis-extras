use deboa::{
    cert::{Certificate as _, ContentEncoding},
    request,
};
use deboa_tokio::cert::DeboaCertificate;
use http::StatusCode;
use http_body_util::BodyExt as _;
use std::error::Error;
use vetis::{virtual_host::VirtualHost as _, Response, VetisServer as _};
use vetis_proxy::{tokio::ProxyPath, ProxyPathConfig};
use vetis_tokio::{
    handler_fn,
    virtual_host::{path::HandlerPath, VirtualHostImpl},
    ListenerConfig, SecurityConfig, ServerConfig, Vetis, VirtualHostConfig,
};

use crate::common::{CA_CERT, SERVER_CERT, SERVER_KEY};

#[tokio::test]
async fn test_get_proxy_to_target() -> Result<(), Box<dyn Error>> {
    let source_listener = ListenerConfig::builder()
        .port(8084)
        .protocol(vetis_tokio::Protocol::Http1)
        .interface("0.0.0.0")
        .build()?;

    let target_listener = ListenerConfig::builder()
        .port(8085)
        .protocol(vetis_tokio::Protocol::Http1)
        .interface("0.0.0.0")
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(source_listener)
        .add_listener(target_listener)
        .build()?;

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let source_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8084)
        .root_directory("src/tests")
        .security(security_config.clone())
        .build()?;

    let mut source_virtual_host = VirtualHostImpl::new(source_config);
    source_virtual_host.add_path(ProxyPath::new(
        ProxyPathConfig::builder()
            .uri("/")
            .target("http://localhost:8085")
            .build()?,
    ));

    let target_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8085)
        .root_directory("src/tests")
        .build()?;

    let mut target_virtual_host = VirtualHostImpl::new(target_config);
    target_virtual_host.add_path(
        HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(|_request| async move {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .text("Hello, world!"))
            }))
            .build()?,
    );

    assert_eq!(
        target_virtual_host
            .config()
            .hostname(),
        "localhost"
    );

    let mut server = Vetis::new(config);
    server
        .add_virtual_host(source_virtual_host)
        .await;
    server
        .add_virtual_host(target_virtual_host)
        .await;

    server
        .start()
        .await?;

    let client = deboa_tokio::Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .protocol(deboa::HttpVersion::Http1)
        .build();

    let request = request::get("https://localhost:8085/")?
        .send_with(&client)
        .await?;

    assert_eq!(request.status(), StatusCode::OK);
    assert_eq!(
        request
            .text()
            .await?,
        "Hello, world!"
    );

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_post_proxy_to_target() -> Result<(), Box<dyn Error>> {
    let source_listener = ListenerConfig::builder()
        .port(9093)
        .protocol(vetis_tokio::Protocol::Http1)
        .interface("0.0.0.0")
        .build()?;

    let target_listener = ListenerConfig::builder()
        .port(9094)
        .protocol(vetis_tokio::Protocol::Http1)
        .interface("0.0.0.0")
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(source_listener)
        .add_listener(target_listener)
        .build()?;

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let source_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(9093)
        .root_directory("src/tests")
        .security(security_config.clone())
        .build()?;

    let mut source_virtual_host = VirtualHostImpl::new(source_config);
    source_virtual_host.add_path(ProxyPath::new(
        ProxyPathConfig::builder()
            .uri("/")
            .target("http://localhost:9094")
            .build()?,
    ));

    let target_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(9094)
        .root_directory("src/tests")
        .build()?;

    let mut target_virtual_host = VirtualHostImpl::new(target_config);
    target_virtual_host.add_path(
        HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(|request| async move {
                let (_parts, body) = request.into_parts();
                let text = body
                    .collect()
                    .await
                    .unwrap()
                    .to_bytes();
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .bytes(text.as_ref()))
            }))
            .build()?,
    );

    assert_eq!(
        target_virtual_host
            .config()
            .hostname(),
        "localhost"
    );

    let mut server = Vetis::new(config);
    server
        .add_virtual_host(source_virtual_host)
        .await;
    server
        .add_virtual_host(target_virtual_host)
        .await;

    server
        .start()
        .await?;

    let client = deboa_tokio::Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .protocol(deboa::HttpVersion::Http1)
        .build();

    let response = request::post("https://localhost:9093/")?
        .text("Something cool!")
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .text()
            .await?,
        "Something cool!"
    );

    server
        .stop()
        .await?;

    Ok(())
}
