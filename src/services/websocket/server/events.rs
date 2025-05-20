use tokio::sync::oneshot::Sender;

use super::server_connection::Connection;
use crate::services::websocket::Message;

pub enum SocketEvents {
    Handshake(Sender<u16>, Connection),
    Disconnect(u32),
    Broadcast {
        message: Message,
        connection_ids: Option<Vec<u32>>,
    },
}
