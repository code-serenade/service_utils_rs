use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Sending GET request to http://127.0.0.1:11434 ...");

    let response = client
        .get("http://127.0.0.1:11434") // 注意用 127.0.0.1，避免 localhost DNS 问题
        .send()
        .await?;

    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

// cargo run --example llm_stream_req  --features request
