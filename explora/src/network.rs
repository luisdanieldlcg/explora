use common::packet::{self, ClientPacket};
use quinn::{ClientConfig, Connection, Endpoint};
use std::thread;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use vek::Vec3;

use crate::config::Config;

#[derive(Debug)]
pub enum Packet {
    Input(Vec3<f32>),
}

pub struct NetworkThread {
    pub send_channel: mpsc::UnboundedSender<Packet>,
    pub thread: thread::JoinHandle<()>,
}

impl NetworkThread {
    pub fn spawn(config: Config) -> Self {
        let (outgoing_send, outgoing_recv) = tokio::sync::mpsc::unbounded_channel();
        let handle = thread::spawn(move || {
            network_thread(&config, outgoing_recv);
        });
        tracing::info!("Network thread spawned");
        Self {
            send_channel: outgoing_send,
            thread: handle,
        }
    }
}

#[tokio::main]
async fn network_thread(cfg: &Config, outgoing_recv: mpsc::UnboundedReceiver<Packet>) {
    let (endpoint, connection) = create_quic_client(cfg.server_addr.unwrap()).await;
    let stream = connection.open_uni().await.unwrap();
    tokio::spawn(handle_outgoing_packets(connection.clone(), outgoing_recv));
    packet::send(stream, ClientPacket::Hello {
        username: (*cfg.username).into(),
    }).await;

    // setup incoming packets
    
    // receive server hello
    let server_hellostream = connection.accept_uni().await.unwrap(); 
    let server_hello: packet::ServerPacket = packet::recv(server_hellostream, 1024).await;
    tracing::info!(?server_hello);
    endpoint.wait_idle().await; // Don't let the connection die until we're done with it
    tracing::info!("Network thread exiting");
}

async fn handle_outgoing_packets(
    connection: Connection,
    mut outgoing_recv: mpsc::UnboundedReceiver<Packet>,
) {
    while let Some(packet) = outgoing_recv.recv().await {
        // let stream = connection.open_uni().await.unwrap();
    }
}

async fn create_quic_client(to: SocketAddr) -> (Endpoint, Connection) {
    let mut endpoint = quinn::Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap();
    use quinn::rustls;

    #[derive(Debug)]
    struct AnyCert;

    impl rustls::client::danger::ServerCertVerifier for AnyCert {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::pki_types::CertificateDer,
            _intermediates: &[rustls::pki_types::CertificateDer],
            _server_name: &rustls::pki_types::ServerName,
            _ocsp_response: &[u8],
            _now: rustls::pki_types::UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            // QUIC is TLS 1.3 only
            unreachable!();
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &rustls::pki_types::CertificateDer<'_>,
            dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            rustls::crypto::verify_tls13_signature(
                message,
                cert,
                dss,
                &rustls::crypto::CryptoProvider::get_default()
                    .unwrap()
                    .signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            rustls::crypto::CryptoProvider::get_default()
                .unwrap()
                .signature_verification_algorithms
                .supported_schemes()
        }
    }

    let crypto_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AnyCert))
        .with_no_client_auth();
    endpoint.set_default_client_config(ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(crypto_config).unwrap(),
    )));

    let conn = endpoint.connect(to, "localhost").unwrap().await.unwrap();
    (endpoint, conn)
}
