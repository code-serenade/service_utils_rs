use service_utils_rs::services::websocket::client::WebSocketClient;
use std::error::Error;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化 WebSocket 客户端并连接到服务器
    let url = "ws://127.0.0.1:13785".to_string();
    let client = WebSocketClient::new(url).await?;

    // 发送一个消息到 WebSocket 服务器
    let msg = "Hello WebSocket".to_string();
    client.send_message(msg).await?;

    // 保持主线程运行一段时间，模拟长时间的 WebSocket 客户端运行
    // 这里设置为 60 秒，你可以根据需求调整
    tokio::time::sleep(Duration::from_secs(25)).await;
    let msg = "Hello WebSocket end".to_string();
    client.send_message(msg).await?;
    tokio::time::sleep(Duration::from_secs(60)).await;
    Ok(())
}
