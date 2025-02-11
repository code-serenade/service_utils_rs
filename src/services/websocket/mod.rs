pub mod client;
pub mod connection;
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
