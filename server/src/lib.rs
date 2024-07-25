use std::{collections::HashMap, net::UdpSocket};

use common::{
    config::GameConfig,
    packet::{self, ClientPacket, ServerPacket},
};
use quinn::{Connection, Endpoint, EndpointConfig, ServerConfig};
use tokio::{select, sync::mpsc};
use tracing::error;

#[derive(Debug)]
pub enum Event {
    PlayerJoined(String),
}

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

pub struct Server {
    clients: HashMap<u32, Connection>,
    last_id: u32,
}

impl Server {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // create server state
        Self {
            clients: HashMap::new(),
            last_id: 0,
        }
    }

    pub async fn run(&mut self, endpoint: Endpoint) {
        let mut incoming_connection = Self::handle_incoming(endpoint);

        loop {
            if let Ok(connection) = incoming_connection.try_recv() {
                self.on_connect(connection).await;
            }
        }
    }

    fn handle_incoming(endpoint: Endpoint) -> mpsc::Receiver<quinn::Connection> {
        let (connection_send, connection_recv) = mpsc::channel(8);
        tokio::spawn(async move {
            while let Some(conn) = endpoint.accept().await {
                match conn.await {
                    Ok(conn) => {
                        let _ = connection_send.send(conn).await;
                    }
                    Err(e) => error!("Incoming connection failed: {:?}", e),
                }
            }
        });
        connection_recv
    }

    async fn on_connect(&mut self, connection: Connection) {
        self.clients.insert(self.last_id, connection.clone());
        self.last_id += 1;

        let stream = connection.accept_uni().await.unwrap();
        let packet = common::packet::recv::<ClientPacket>(stream, 256).await;
        if let ClientPacket::Hello { username } = packet {
            tracing::info!("{username} has joined");
        }

        tokio::spawn(async move {
            loop {
                if let Ok(packetstream) = connection.accept_uni().await {
                    match common::packet::recv::<ClientPacket>(packetstream, 256).await {
                        ClientPacket::Hello { username } => (),
                        ClientPacket::BlockPosUpdate(pos) => {
                            tracing::info!("Player moved to: {pos}");
                        }
                    }
                }
            }
        });
    }
}
