use std::{net::SocketAddr, sync::Arc};

use quinn::{ClientConfig, Connection, Endpoint};
use tokio::runtime::Runtime;

pub struct Client {
    rt: Runtime,
    endpoint: Endpoint,
    remote_connection: Connection,
}

impl Client {
    pub fn new(remote_addr: SocketAddr) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _guard = rt.enter();

        let net = rt.block_on(create_quic_client(remote_addr));
        tracing::info!("Client created");
        Self {
            rt,
            endpoint: net.0,
            remote_connection: net.1,
        }
    }

    pub fn tick(&mut self) {
        tracing::info!("Client tick");
    }
}

async fn create_quic_client(to: SocketAddr) -> (Endpoint, Connection) {
    let mut endpoint = quinn::Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap();

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
    let crypto_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(AnyCertificate))
        .with_no_client_auth();
    endpoint.set_default_client_config(ClientConfig::new(Arc::new(crypto_config)));

    let conn = endpoint.connect(to, "localhost").unwrap().await.unwrap();
    (endpoint, conn)
}
