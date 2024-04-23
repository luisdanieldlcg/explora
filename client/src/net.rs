use std::sync::Arc;

use bytes::Bytes;
use quinn::{ClientConfig, ConnectError, Connection};

use crate::config::Config;

#[tokio::main]
pub async fn setup(config: Config) -> Result<(), ConnectError> {
    tracing::info!(?config);
    let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap()).unwrap();
    let crypto_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(AnyCertificate))
        .with_no_client_auth();

    endpoint.set_default_client_config(ClientConfig::new(Arc::new(crypto_config)));

    let connection = endpoint
        .connect(config.server_addr.unwrap(), "localhost")?
        .await
        .unwrap();

    connection
        .send_datagram(Bytes::from_static(b"Hello, world!"))
        .unwrap();

    endpoint.wait_idle().await;
    Ok(())
}

struct AnyCertificate;

impl rustls::client::ServerCertVerifier for AnyCertificate {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
