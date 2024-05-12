use std::{
    net::{SocketAddr, UdpSocket},
    thread::JoinHandle,
};

use quinn::{
    rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    ServerConfig,
};

use crate::config::Config;

// Used to manage the integrated server of a singleplayer game
pub struct Singleplayer {
    server_thread: JoinHandle<()>,
}

impl Singleplayer {
    pub fn new(config: &mut Config) -> Self {
        tracing::info!("Starting singleplayer server");
        // Setup local server
        let certified_key =
            rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();

        let (key, cert) = {
            let key = certified_key.key_pair.serialize_der();
            let cert = certified_key.cert.der().to_vec();
            (
                PrivateKeyDer::from(PrivatePkcs8KeyDer::from(key)),
                vec![CertificateDer::from(cert)],
            )
        };
        let socket = UdpSocket::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        config.server_addr = Some(socket.local_addr().unwrap());
        let server_thread = std::thread::spawn(|| run_integrated_server(socket, cert, key));
        Self { server_thread }
    }
}

fn run_integrated_server(
    socket: UdpSocket,
    certificate_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) {
    tracing::info!("Starting integrated server...");
    server::init(
        socket,
        ServerConfig::with_single_cert(certificate_chain, key).unwrap(),
    );
}
