use super::connection::Connection;
// use bytes::BytesMut;
// use tokio::sync::mpsc::Sender;

pub enum SocketEvents {
    Handshake(Connection),
    Disconnect(u32),
}
