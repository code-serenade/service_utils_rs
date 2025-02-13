pub mod client;
pub mod client_router;
pub mod connection;
pub mod handler;
pub mod server;

enum ClientEvents {
    Reconnect,
    SendMessage(Message),
}

type Message = tokio_tungstenite::tungstenite::Message;
type ClientSender = tokio::sync::mpsc::Sender<ClientEvents>;
type ClientReciver = tokio::sync::mpsc::Receiver<ClientEvents>;

type MsgSender = tokio::sync::mpsc::Sender<Message>;
type MsgReciver = tokio::sync::mpsc::Receiver<Message>;
