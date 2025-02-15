use axum::Router;
use service_utils_rs::error::Result;

use service_utils_rs::services::http::http_server;
use service_utils_rs::settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new("examples/config/services.toml").unwrap();
    let router = Router::new();
    http_server::start(settings.http.port, router)
        .await
        .unwrap();

    Ok(())
}

// cargo run --example http_server_example
