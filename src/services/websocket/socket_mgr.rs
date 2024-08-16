use crate::error::{Error, Result};

use bytes::BytesMut;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;

use super::{connection::Connection, socket_events::SocketEvents};

pub struct SocketMgr {
    connections: HashMap<u32, Connection>,
    max_client: u32,
    connection_indices: u32,
    connection_index_pool: Vec<u32>,
}

impl SocketMgr {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            max_client: 1000,
            connection_indices: 0,
            connection_index_pool: Vec::new(),
        }
    }

    pub fn add_connection(&mut self) -> Result<u32> {
        let id = if !self.connection_index_pool.is_empty() {
            self.connection_index_pool.pop().unwrap()
        } else if self.connection_indices < self.max_client {
            self.connection_indices += 1;
            self.connection_indices
        } else {
            0
        };

        if id == 0 {
            return Err(Error::ErrorCode(1));
        }

        Ok(id)
    }

    pub fn remove_connection(&mut self, id: &u32) {
        println!("===============remove connection=========: {}", id);
        // self.connections.remove(id).unwrap();
    }

    /// 广播消息给指定或所有连接的客户端
    pub async fn broadcast_message(
        &self,
        error_code: u16,
        cmd: u16,
        message: Option<BytesMut>,
        connection_ids: Option<Vec<u32>>,
    ) {
        match connection_ids {
            Some(ids) => {
                // 发送给指定的连接
                for id in ids {
                    if let Some(connection) = self.connections.get(&id) {
                        if let Err(e) = connection
                            .msg_sender
                            .send((error_code, cmd, message.clone()))
                            .await
                        {
                            eprintln!("Failed to send message to connection {}: {}", id, e);
                        }
                    }
                }
            }
            None => {
                // 发送给所有连接
                for (id, connection) in &self.connections {
                    if let Err(e) = connection
                        .msg_sender
                        .send((error_code, cmd, message.clone()))
                        .await
                    {
                        eprintln!("Failed to send message to connection {}: {}", id, e);
                    }
                }
                println!(
                    "Broadcast message sent to {} clients",
                    self.connections.len()
                );
            }
        }
    }
}

pub async fn start_loop(mut reciever: UnboundedReceiver<SocketEvents>) {
    let mut mgr = SocketMgr::new();
    while let Some(event) = reciever.recv().await {
        match event {
            SocketEvents::Handshake(mut conn) => match mgr.add_connection() {
                Ok(id) => {
                    conn.id = id;
                    mgr.connections.insert(id, conn);

                    // if tx.send(id).is_ok() {
                    //     mgr.broadcast_lobby_info().await;
                    // } else {
                    //     mgr.connections.remove(&id).unwrap();
                    // }
                }
                Err(Error::ErrorCode(code)) => {
                    let _ = conn.msg_sender.send((code, 0, None));
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            SocketEvents::Disconnect(id) => mgr.remove_connection(&id),
        }
    }
}
