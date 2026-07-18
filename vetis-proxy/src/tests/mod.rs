use crate::ProxyPathConfig;

mod path;

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

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
