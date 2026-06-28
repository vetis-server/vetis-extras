use crate::tests::vetis_default_protocol;
use std::error::Error;
use vetis::errors::{ConfigError, VetisError};
use vetis::virtual_host::path::proxy::ProxyPathConfig;
use vetis::{
    listener::ListenerConfig, security::SecurityConfig, server::ServerConfig,
    virtual_host::VirtualHostConfig,
};

#[test]
fn test_reverse_proxy_config() -> Result<(), Box<dyn std::error::Error>> {
    let reverse_proxy_config = ProxyPathConfig::builder()
        .uri("/")
        .target("http://localhost:8081")
        .build()?;
    assert_eq!(reverse_proxy_config.uri(), "/");
    assert_eq!(reverse_proxy_config.target(), "http://localhost:8081");
    Ok(())
}
