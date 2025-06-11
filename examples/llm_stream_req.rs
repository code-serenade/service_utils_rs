use std::{io::Write, time::Duration};

use futures_util::StreamExt;
use serde_json::json;
use service_utils_rs::{error::Result, utils::Request};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个新的 Request 实例
    let mut client = Request::new();

    // 设置 Ollama 流式请求 base_url
    client.set_base_url("http://localhost:11434")?;
    let stream_headers = vec![("Content-Type", "application/json".to_string())];
    client.set_default_headers(stream_headers)?;

    // 构造 Ollama 请求体
    let stream_body = json!({
        "model": "llama3.2",
        "stream": true,
        "messages": [
            {"role": "user", "content": "Hello, who are you?"}
        ]
    });

    let mut stream = client.post_stream("api/chat", &stream_body, None).await?;

    println!("Streaming Response:");
    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        let s = std::str::from_utf8(&data).unwrap();

        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(content) = json["message"]["content"].as_str() {
                        print!("{}", content);
                        std::io::stdout().flush().unwrap();
                    }
                    if json["done"] == true {
                        println!();
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }

        // 可选：稍作等待，避免拉取过快影响显示
        sleep(Duration::from_millis(20)).await;
    }

    Ok(())
}

// cargo run --example llm_stream_req --features request
