use std::sync::Arc;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self},
        oneshot,
    },
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::{server_router::ServerRouter, SocketEventSender};
use crate::{
    error::{Error, Result},
    services::websocket::{server::events::SocketEvents, JsonMessage, MsgReciver, MsgSender},
};

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
                let parsed_msg: JsonMessage =
                    serde_json::from_slice(&bin).map_err(|e| Error::ErrorMessage(e.to_string()))?;
                tokio::spawn(process_message(
                    parsed_msg,
                    router.clone(),
                    tx.clone(),
                    socket_event_sender.clone(),
                ));
            }
            Ok(Message::Ping(_ping)) => {}
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => {
                // println!("Client closed the connection");
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
    message: JsonMessage,
    router: Arc<ServerRouter>,
    tx: MsgSender,
    socket_event_sender: SocketEventSender,
) -> Result<()> {
    match router
        .handle_message(&message.action, message.data, socket_event_sender)
        .await
    {
        Some(response) => {
            let bin =
                serde_json::to_vec(&response).map_err(|e| Error::ErrorMessage(e.to_string()))?;
            let message = Message::binary(bin);
            tx.send(message)
                .await
                .map_err(|e| Error::ErrorMessage(e.to_string()))?;
        }
        None => {}
    }
    Ok(())
}
