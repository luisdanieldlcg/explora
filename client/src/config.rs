use std::net::{SocketAddr};

#[derive(Debug, Default)]
// TODO: create default config and load from file
pub struct Config {
    pub server_addr: Option<SocketAddr>,
}

impl Config {

    pub fn hardcoded() -> Self {
        Self {
            server_addr: Some("127.0.0.1:60123".parse().unwrap()),
        }
    }
        
}