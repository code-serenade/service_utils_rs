mod common;
use std::sync::Arc;

use common::Settings;
use service_utils_rs::{
    error::Result,
    services::{
        jwt::Jwt,
        websocket::{
            self, JsonMessage,
            server::{SocketEventSender, server_router::ServerRouter},
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::load("examples/config/services.toml").unwrap();
    let jwt = Jwt::new(settings.jwt);

    let router = init_router();
    let router = Arc::new(router);

    let token_validator = move |token: &str| -> u32 {
        match jwt.validate_access_token(token) {
            Ok(claims) => sub_to_id(&claims.sub),
            Err(_) => 0,
        }
    };

    websocket::server::start(settings.websocket.port, router, token_validator)
        .await
        .unwrap();
    Ok(())
}

fn init_router() -> ServerRouter {
    let mut router = ServerRouter::new();
    router
        .add_route("test1", handle_user_info)
        .add_route("test", handle_order);
    router
}

async fn handle_user_info(data: serde_json::Value, _tx: SocketEventSender) -> Option<JsonMessage> {
    // todo others
    println!("data: {:?}", data);
    // let response = BytesMut::from("User Info: John Doe");
    None
}

// 定义另一个处理函数
async fn handle_order(data: serde_json::Value, _tx: SocketEventSender) -> Option<JsonMessage> {
    let msg = JsonMessage {
        action: "test".to_string(),
        data: serde_json::json!({"name": "test"}),
    };
    println!("got order: {:?}", data);
    Some(msg)
}

fn sub_to_id(sub: &str) -> u32 {
    match sub.parse::<u32>() {
        Ok(id) => id,
        Err(_) => 300,
    }
}

// cargo run --example socket_server --features full
