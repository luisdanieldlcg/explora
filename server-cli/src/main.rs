use std::net::{SocketAddr, UdpSocket};

use quinn::rustls::pki_types::CertificateDer;
use quinn::rustls::pki_types::PrivateKeyDer;
use quinn::rustls::pki_types::PrivatePkcs8KeyDer;
use quinn::ServerConfig;

fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Generating self signed certificate");
    let certified_key = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();

    let (key, cert) = {
        let key = certified_key.key_pair.serialize_der();
        let cert = certified_key.cert.der().to_vec();
        (
            PrivateKeyDer::from(PrivatePkcs8KeyDer::from(key)),
            vec![CertificateDer::from(cert)],
        )
    };

    let config = ServerConfig::with_single_cert(cert, key).unwrap();
    tracing::info!("Created server config");

    let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(addr).unwrap();
    server::init(socket, config);
}
