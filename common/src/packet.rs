use crate::config::GameConfig;
use quinn::SendStream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Packets send from the client to the server
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Hello { username: String },
}

/// Packets send from the server to the client
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPacket {
    Hello { player_id: u32, config: GameConfig },
}

// the streams are consumed by the send and recv functions.
// no need to call .finish() on them

pub async fn send<T: Serialize>(mut sendstream: SendStream, packet: T) {
    let data = bincode::serialize(&packet).unwrap();
    sendstream.write_all(&data).await.unwrap();
}

pub async fn recv<T: DeserializeOwned>(mut recvstream: quinn::RecvStream, size_limit: usize) -> T {
    let buf = recvstream.read_to_end(size_limit).await.unwrap();
    bincode::deserialize(&buf).unwrap()
}
