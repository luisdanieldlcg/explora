use std::{collections::HashMap, net::UdpSocket};

use common::{config::GameConfig, packet::{self, ClientPacket, ServerPacket}};
use quinn::{Connection, Endpoint, EndpointConfig, ServerConfig};
use tokio::{select, sync::mpsc};
use tracing::error;

#[derive(Debug)]
pub enum Event {
    PlayerJoined(String)
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
    pub fn new() -> Self {
        // create server state
        Self {
            clients: HashMap::new(),
            last_id: 0,
        }
    }

    pub async fn run(&mut self, endpoint: Endpoint) {
        let mut incoming_connection = Self::handle_incoming(endpoint);
        let (event_send, mut event_recv) = mpsc::channel(128);
        loop {
            select! {
                connection = incoming_connection.recv() => {
                    if let Some(connection) = connection {
                        self.on_connect(connection, event_send.clone());
                    }
                }
                event = event_recv.recv() => {
                    if let Some((id, event)) = event {
                        self.on_event(id, event);
                    }
                }
            }
            self.update();
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

    fn update(&mut self) {}

    fn on_connect(&mut self, connection: Connection, sender: mpsc::Sender<(u32, Event)>) {
        tracing::info!("Handling connection");
        self.clients.insert(self.last_id, connection.clone());

        let id = self.last_id;
        tokio::spawn(async move {
            let client_hellostream = connection.accept_uni().await.unwrap();
            let ClientPacket::Hello { username } = packet::recv(client_hellostream, 1 << 16).await;
            sender.send((id,Event::PlayerJoined(username))).await.unwrap();

            let server_hellostream = connection.open_uni().await.unwrap();
            packet::send(server_hellostream, ServerPacket::Hello {
                player_id: id,
                config: GameConfig::default(),
            }).await;

        });
        self.last_id += 1;
    }

    fn on_event(&mut self, id: u32, events: Event) {

        // check the client actually exists
        let Some(client) = self.clients.get(&id) else {
            return;
        };

        match events {
            Event::PlayerJoined(username) => {
                tracing::info!("{:?} has joined the game.", username);
                let conn = client.clone();

                tokio::spawn(async move {
                    let mut stream = conn.open_uni().await.unwrap();

                    
                });
            }
        }
    }
}
