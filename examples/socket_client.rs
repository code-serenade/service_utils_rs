use std::{error::Error, sync::Arc};

use service_utils_rs::{
    add_handler,
    services::websocket::{
        JsonMessage,
        client::{WebSocketClient, client_router::ClientRouter},
    },
};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化 WebSocket 客户端并连接到服务器
    let url = "ws://localhost:18123/?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.\
               eyJhdWQiOiJ0ZXN0Iiwic3ViIjoiMiIsImV4cCI6MTczOTk1NTA3MCwiaWF0IjoxNzM5MzUwMjcwfQ.\
               uOc5-2ACjyZPY5BbwiqGYCkCzNz84SocT0Tc2NKZITo"
        .to_string();
    let mut router = ClientRouter::new();
    add_handler!(router, "test", Test, hh);
    let r = Arc::new(router);
    let client = WebSocketClient::new(url, r).await?;
    let c = Arc::new(client);

    // 发送一个消息到 WebSocket 服务器
    tokio::spawn(send_json(c.clone()));

    // 保持主线程运行一段时间，模拟长时间的 WebSocket 客户端运行
    // 这里设置为 60 秒，你可以根据需求调整
    tokio::time::sleep(Duration::from_secs(25)).await;

    tokio::spawn(send_msg(c.clone(), "Hello WebSocket end".to_string()));
    // let msg = "Hello WebSocket end".to_string();
    // c.send_text_message(msg).await?;
    tokio::time::sleep(Duration::from_secs(60)).await;
    Ok(())
}

async fn send_msg(c: Arc<WebSocketClient>, msg: String) {
    c.send_text_message(msg).await.unwrap();
}

async fn send_json(c: Arc<WebSocketClient>) {
    let msg = JsonMessage {
        action: "test".to_string(),
        data: serde_json::json!({"name": "test"}),
    };
    c.send_json_message(msg).await.unwrap();
}

async fn hh(data: Test) -> Option<JsonMessage> {
    println!("Received data: {:?}", data);
    // let test: Test = serde_json::from_value(data).unwrap();
    // println!("{:?}", test);
    let msg = JsonMessage {
        action: "gg334".to_string(),
        data: serde_json::json!({"name": "test"}),
    };
    Some(msg)
}

#[derive(Debug, serde::Deserialize)]
pub struct Test {
    pub name: String,
}

// cargo run --example socket_client --features websocket
