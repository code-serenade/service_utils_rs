use super::{
    connection::Connection,
    error_code::{SUCCECE, SYSTEM_ERROR},
    events::SocketEvents,
};
use crate::{
    error::{Error, Result},
    services::websocket::{Message, MsgSender},
};

use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct SocketManager {
    connections: HashMap<u32, Connection>,
    max_client: usize,
}

impl SocketManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            max_client: 10000,
        }
    }

    pub fn add_connection(&mut self, conn: Connection) -> Result<()> {
        if self.connections.len() >= self.max_client {
            return Err(Error::SystemError("Too many connections".to_string()));
        }
        self.connections.insert(conn.id, conn);

        Ok(())
    }

    pub fn remove_connection(&mut self, id: u32) {
        if self.connections.remove(&id).is_none() {
            println!("Connection {} already removed or not found", id);
        }
    }

    /// 广播消息给指定或所有连接的客户端
    pub async fn broadcast_message(&self, message: Message, connection_ids: Option<Vec<u32>>) {
        // 收集目标连接
        let target_connections: Vec<(u32, MsgSender)> = match connection_ids {
            Some(ids) => ids
                .into_iter()
                .filter_map(|id| {
                    self.connections
                        .get(&id)
                        .map(|conn| (id, conn.msg_sender.clone()))
                })
                .collect(),
            None => self
                .connections
                .iter()
                .map(|(id, conn)| (*id, conn.msg_sender.clone()))
                .collect(),
        };

        // 广播消息
        for (id, msg_sender) in target_connections {
            let message = message.clone();
            tokio::spawn(async move {
                if let Err(e) = msg_sender.send(message).await {
                    eprintln!("Failed to send message to connection {}: {}", id, e);
                }
            });
        }
    }
}

pub async fn start_loop(mut reciever: UnboundedReceiver<SocketEvents>) -> Result<()> {
    let mut mgr = SocketManager::new();
    while let Some(event) = reciever.recv().await {
        match event {
            SocketEvents::Handshake(tx, conn) => match mgr.add_connection(conn) {
                Err(_e) => {
                    tx.send(SYSTEM_ERROR).unwrap();
                }
                Ok(_) => tx.send(SUCCECE).unwrap(),
            },
            SocketEvents::Broadcast {
                message,
                connection_ids,
            } => {
                mgr.broadcast_message(message, connection_ids).await;
            }
            SocketEvents::Disconnect(id) => mgr.remove_connection(id),
        }
    }
    Ok(())
}
