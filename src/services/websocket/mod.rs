pub mod client;
pub mod connection;
pub mod server;

type MsgSender = tokio::sync::mpsc::Sender<String>;
