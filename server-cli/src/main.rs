use std::net::{SocketAddr, UdpSocket};

use quinn::ServerConfig;
fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Generating self signed certificate");
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let key = rustls::PrivateKey(cert.serialize_private_key_der());
    let cert = cert.serialize_der().unwrap();

    let config = ServerConfig::with_single_cert(vec![rustls::Certificate(cert)], key).unwrap();
    tracing::info!("Created server config");

    let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(addr).unwrap();
    server::init(socket, config);
}
