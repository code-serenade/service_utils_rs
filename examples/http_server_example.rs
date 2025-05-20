use axum::Router;
use service_utils_rs::{error::Result, services::http::http_server, settings::Settings};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::load("examples/config/services.toml").unwrap();
    let router = Router::new();
    http_server::start(settings.http.port, router)
        .await
        .unwrap();

    Ok(())
}

// cargo run --example http_server_example --features http
