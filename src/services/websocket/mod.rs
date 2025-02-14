pub mod client;
pub mod connection;
pub mod handler;
pub mod server;

type Message = tokio_tungstenite::tungstenite::Message;
type MsgSender = tokio::sync::mpsc::Sender<Message>;
type MsgReciver = tokio::sync::mpsc::Receiver<Message>;
