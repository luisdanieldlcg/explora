use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub server_addr: Option<SocketAddr>,
    pub singleplayer: bool,
}

impl Config {
    pub fn load() -> Self {
        let file_contents = std::fs::read_to_string("config.toml").unwrap();
        let cfg = toml::from_str::<Self>(&file_contents).unwrap();
        tracing::info!(?cfg, "Loaded config");
        cfg
    }
}
