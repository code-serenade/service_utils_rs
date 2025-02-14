use serde::{Deserialize, Serialize};

pub mod client;
pub mod handler;
pub mod server;

type Message = tokio_tungstenite::tungstenite::Message;
type MsgSender = tokio::sync::mpsc::Sender<Message>;
type MsgReciver = tokio::sync::mpsc::Receiver<Message>;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMessage {
    pub action: String,
    pub data: serde_json::Value,
}
