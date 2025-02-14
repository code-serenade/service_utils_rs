use super::{server_router::ServerRouter, SocketEventSender};
use crate::{
    error::{Error, Result},
    services::websocket::{
        server::{events::SocketEvents, server_router::IncomingMessage},
        MsgReciver, MsgSender,
    },
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self},
        oneshot,
    },
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// Alias for the writing half of a WebSocket connection.
type SocketWriter = SplitSink<WebSocketStream<TcpStream>, Message>;
/// Alias for the reading half of a WebSocket connection.
type SocketReader = SplitStream<WebSocketStream<TcpStream>>;

/// Represents a client connection.
#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub msg_sender: MsgSender,
}

impl Connection {
    pub fn new(id: u32, msg_sender: MsgSender) -> Self {
        Self { id, msg_sender }
    }
}

pub async fn handle_connection(
    router: Arc<ServerRouter>,
    ws_stream: WebSocketStream<TcpStream>,
    sender: SocketEventSender,
    id: u32,
) {
    println!("socket id: {}", id);

    // Message channel
    let (msg_sender, msg_reciever) = mpsc::channel::<Message>(4);

    let (tx, rx) = oneshot::channel::<u16>();

    let connection = Connection::new(id, msg_sender.clone());

    let conn_id = connection.id;

    // Send a handshake event to the connection manager
    sender
        .send(SocketEvents::Handshake(tx, connection))
        .unwrap();

    process_handshake(
        rx,
        ws_stream,
        router,
        msg_sender,
        sender,
        conn_id,
        msg_reciever,
    )
    .await
    .unwrap();
}

async fn process_handshake(
    rx: oneshot::Receiver<u16>,
    ws_stream: WebSocketStream<TcpStream>,
    router: Arc<ServerRouter>,
    msg_sender: MsgSender,
    socket_event_sender: SocketEventSender,
    connection_id: u32,
    msg_reciever: MsgReciver,
) -> Result<()> {
    match rx.await {
        Ok(error_code) => match error_code {
            0 => {
                let (socket_writer, socket_reader) = ws_stream.split();
                tokio::spawn(recieve_msg(msg_reciever, socket_writer));
                handle_msg(
                    router,
                    socket_reader,
                    msg_sender,
                    socket_event_sender,
                    connection_id,
                )
                .await
            }
            _ => Ok(()),
        },
        Err(_) => Ok(()),
    }
}

async fn recieve_msg(mut rx: MsgReciver, mut writer: SocketWriter) -> Result<()> {
    while let Some(msg) = rx.recv().await {
        if let Err(e) = writer.send(msg).await {
            eprintln!("Error sending message: {}", e);
            return Err(Error::WsError(e));
        }
    }
    println!("recieve_msg task is exiting due to connection drop or other error.");
    Ok(())
}

async fn handle_msg(
    router: Arc<ServerRouter>,
    mut read: SocketReader,
    tx: MsgSender,
    socket_event_sender: SocketEventSender,
    connection_id: u32,
) -> Result<()> {
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(msg)) => {
                println!("Received text message: {}", msg);
            }
            Ok(Message::Binary(bin)) => {
                println!("Received binary message");
                // if let Err(e) = write.send(Message::Binary(msg)).await {
                //     eprintln!("Error sending message: {}", e);
                //     return;
                // }
            }
            Ok(Message::Ping(ping)) => {
                println!("Received ping");
                let pong = Message::Pong(ping);
                tx.send(pong).await.unwrap();
            }
            Ok(Message::Pong(_)) => {
                println!("Received pong");
            }
            Ok(Message::Close(_)) => {
                println!("Client closed the connection");
                break;
            }
            Ok(Message::Frame(_)) => {
                println!("Received frame message");
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        };
    }

    println!("WebSocket connection closed");
    socket_event_sender
        .send(SocketEvents::Disconnect(connection_id))
        .map_err(|e| Error::CustomError {
            message: format!("Failed to send disconnect event: {}", e),
            line: line!(),
            column: column!(),
        })?;

    Ok(())
}

async fn process_message(
    message: IncomingMessage,
    router: Arc<ServerRouter>,
    _tx: MsgSender,
    socket_event_sender: SocketEventSender,
) {
    router.handle(&message.action, message.data, socket_event_sender)

    // match router.handle_message(message, socket_event_sender).await {
    //     Ok(response_data) => {
    //         if let Err(e) = tx.send((0, cmd, Some(response_data))).await {
    //             eprintln!("Error sending processed message: {}", e);
    //         }
    //     }
    //     Err(_) => {
    //         let error_code = 1;
    //         if let Err(e) = tx.send((error_code, cmd, None)).await {
    //             eprintln!("Error sending processed message: {}", e);
    //         }
    //     }
    // }
}
