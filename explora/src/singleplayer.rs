use std::{
    net::{SocketAddr, UdpSocket},
    thread::JoinHandle,
};

use quinn::ServerConfig;
use rustls::Certificate;

use crate::config::Config;

// Used to manage the integrated server of a singleplayer game
pub struct Singleplayer {
    server_thread: JoinHandle<()>,
}

impl Singleplayer {
    pub fn new(config: &mut Config) -> Self {
        tracing::info!("Starting singleplayer server");
        // Setup local server
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let key = rustls::PrivateKey(cert.serialize_private_key_der());
        let cert = cert.serialize_der().unwrap();
        let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let socket = UdpSocket::bind(addr).unwrap();
        config.server_addr = Some(socket.local_addr().unwrap());
        let cert_chain = vec![Certificate(cert)];
        let server_thread = std::thread::spawn(|| run_integrated_server(socket, cert_chain, key));
        Self { server_thread }
    }
}

fn run_integrated_server(
    socket: UdpSocket,
    certificate_chain: Vec<Certificate>,
    key: rustls::PrivateKey,
) {
    tracing::info!("Starting integrated server...");
    server::init(
        socket,
        ServerConfig::with_single_cert(certificate_chain, key).unwrap(),
    );
}
