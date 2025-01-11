use indexmap::IndexMap;
use serde_json::json;
use service_utils_rs::{error::Result, services::http_client::HttpClient};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个新的 HttpClient 实例
    let mut client = HttpClient::new();

    // 设置 base_url
    client.set_base_url("https://jsonplaceholder.typicode.com")?;

    // 设置默认的请求头
    let mut default_headers = HashMap::new();
    default_headers.insert("Content-Type", "application/json".to_string());
    client.set_default_headers(default_headers)?;

    // 定义自定义请求头
    let mut custom_headers = IndexMap::new();
    custom_headers.insert("Authorization", "Bearer some_token".to_string());

    // 创建一个 POST 请求体
    let body = json!({
        "title": "foo",
        "body": "bar",
        "userId": 1
    });

    // 发送 POST 请求
    let response = client
        .post("/posts", &body, Some(custom_headers.clone()))
        .await?;

    // 打印返回的状态码和响应体
    println!("POST Response: {:?}", response.status());
    let response_body = response.text().await?;
    println!("Response Body: {}", response_body);

    // 发送 GET 请求
    let response = client.get("/posts/1", None, Some(custom_headers)).await?;

    // 打印返回的状态码和响应体
    println!("GET Response: {:?}", response.status());
    let response_body = response.text().await?;
    println!("Response Body: {}", response_body);

    Ok(())
}

// cargo run --example http_example
