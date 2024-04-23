use std::net::UdpSocket;

use quinn::{Endpoint, EndpointConfig, ServerConfig};

#[tokio::main]
pub async fn init(socket: UdpSocket, config: ServerConfig) {
    let endpoint = Endpoint::new(
        EndpointConfig::default(),
        Some(config),
        socket,
        quinn::default_runtime().unwrap(),
    )
    .unwrap();
    tracing::info!(address=%endpoint.local_addr().unwrap(), "listening");
    let mut server = Server::new();
    server.run(endpoint).await;
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        // create server state
        Self {}
    }

    pub async fn run(&mut self, endpoint: Endpoint) {
        loop {
            while let Some(incoming) = endpoint.accept().await {
                tracing::info!("Incoming connection");
                if let Ok(conn) = incoming.await {
                    tracing::info!("Connection established");
                    let f = conn.read_datagram().await;
                    match f {
                        Ok(bytes) => {
                            tracing::info!(data=%String::from_utf8_lossy(&bytes), "Received data");
                        }
                        Err(e) => {
                            tracing::error!(error=%e, "Failed to read datagram");
                        }
                    }
                }
            }
        }
    }
}
