use crate::error::{Error, Result};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use std::time::Duration;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    time,
};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use super::connection::ClientConnection;

type SocketReader = SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;
type SocketWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
/// Alias for the reading half of a WebSocket connection.

type MsgReciver = Receiver<String>;
/// WebSocket 客户端结构体
pub struct WebSocketClient {
    rt: Sender<String>,
}

impl WebSocketClient {
    /// 创建一个新的 WebSocket 客户端
    pub async fn new(url: String) -> Result<Self> {
        let (rt, rx) = mpsc::channel::<String>(4);
        let connection = connect(&url, rt.clone()).await?;
        let rt_clone = rt.clone();
        let client = Self { rt: rt_clone };
        // let mut client_clone = client.clone();
        tokio::spawn(async move {
            let _ = handle_reconnect(url, connection, rx, rt.clone()).await;
        });
        Ok(client)
    }

    /// 发送消息到 WebSocket 服务器
    pub async fn send_message(&self, msg: String) -> Result<()> {
        self.rt
            .send(msg)
            .await
            .map_err(|e| Error::ErrorMessage(e.to_string()))?;
        Ok(())
    }
}

async fn receive_message(
    mut socket_reader: SocketReader,
    rt: Sender<String>,
    exit_tx_send_msg: Sender<()>,
    exit_tx_ping: Sender<()>,
) -> Result<()> {
    loop {
        match socket_reader.next().await {
            Some(Ok(Message::Text(text))) => {
                println!("收到消息: {}", text);
            }
            Some(Ok(Message::Binary(_))) => {
                println!("收到二进制消息");
            }
            Some(Ok(Message::Ping(_))) => {
                println!("收到 Ping 消息");
            }
            Some(Ok(Message::Pong(_))) => {
                println!("收到 Pong 消息");
            }
            Some(Ok(Message::Close(_))) => {
                println!("连接关闭");
                break;
            }
            Some(Ok(Message::Frame(_))) => {
                println!("收到帧消息");
            }
            Some(Err(e)) => {
                println!("接收消息时出错: {}", e);

                // 发送退出信号，通知其他进程退出
                let _ = exit_tx_send_msg.send(()); // 发送退出信号给 handle_send_msg
                let _ = exit_tx_ping.send(()); // 发送退出信号给 send_ping
                let _ = rt
                    .send("reconnect".to_string())
                    .await
                    .map_err(|e| Error::ErrorMessage(e.to_string()));
                break;
            }
            None => {
                println!("没有更多的消息");
                break;
            }
        }
    }
    Ok(())
}

pub async fn connect(url: &str, rt: Sender<String>) -> Result<ClientConnection> {
    let (socket, _) = connect_async(url).await?;
    let (socket_writer, socket_reader) = socket.split();

    let (msg_sender, msg_reciever) = mpsc::channel::<String>(4);
    let (exit_tx_send_msg, exit_rx_send_msg) = mpsc::channel::<()>(1); // 发送消息任务退出通道
    let (exit_tx_ping, exit_rx_ping) = mpsc::channel::<()>(1); // Ping任务退出通道

    // 启动接收消息任务
    tokio::spawn(receive_message(
        socket_reader,
        rt.clone(),
        exit_tx_send_msg.clone(),
        exit_tx_ping.clone(),
    ));

    // 启动发送消息任务，传递退出通道
    tokio::spawn(handle_send_msg(
        msg_reciever,
        socket_writer,
        exit_rx_send_msg,
    ));

    // 启动 Ping 任务，传递退出通道
    tokio::spawn(send_ping(
        msg_sender.clone(),
        Duration::from_secs(10),
        exit_rx_ping,
    ));

    let connection = ClientConnection { msg_sender };
    println!("成功连接到 WebSocket 服务器");
    Ok(connection)
}

async fn handle_send_msg(
    mut rx: MsgReciver,
    mut writer: SocketWriter,
    mut exit_rx: Receiver<()>,
) -> Result<()> {
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                if let Err(e) = writer.send(Message::text(msg)).await {
                    eprintln!("Error sending message: {}", e);
                    return Err(Error::WsError(e));
                }
            }
            _ = exit_rx.recv() => {
                println!("Received exit signal, exiting send message task.");
                break;  // 退出任务
            }
        }
    }
    Ok(())
}

async fn send_ping(
    msg_sender: super::MsgSender,
    interval: Duration,
    mut exit_rx: Receiver<()>,
) -> Result<()> {
    let mut interval_timer = time::interval(interval);

    loop {
        tokio::select! {
            _ = interval_timer.tick() => {
                msg_sender
                    .send("ping".to_string())
                    .await
                    .map_err(|e| Error::ErrorMessage(e.to_string()))?;
            }
            _ = exit_rx.recv() => {
                println!("Received exit signal, exiting ping task.");
                break;  // 退出任务
            }
        }
    }
    Ok(())
}

async fn handle_reconnect(
    url: String,
    mut connection: ClientConnection,
    mut rx: Receiver<String>,
    rt: Sender<String>,
) -> Result<()> {
    while let Some(msg) = rx.recv().await {
        if msg == "reconnect" {
            connection = reconnect(&url, rt.clone()).await?;
        } else {
            connection
                .msg_sender
                .send(msg)
                .await
                .map_err(|e| Error::ErrorMessage(e.to_string()))?;
        }
    }
    Ok(())
}

async fn reconnect(url: &str, rt: Sender<String>) -> Result<ClientConnection> {
    let mut retries = 5; // 最大重连次数
    while retries > 0 {
        match connect(url, rt.clone()).await {
            Ok(connection) => {
                println!("重连成功");
                return Ok(connection); // 成功重连后返回
            }
            Err(e) => {
                retries -= 1;
                println!("重连失败，剩余重试次数: {}, 错误: {}", retries, e);
                if retries > 0 {
                    time::sleep(Duration::from_secs(5)).await; // 失败时等待 5 秒后重试
                }
            }
        }
    }
    Err(Error::ErrorMessage(
        "重连失败，已达最大重试次数".to_string(),
    ))
}
