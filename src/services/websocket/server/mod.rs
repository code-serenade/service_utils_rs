pub mod error_code;
pub mod events;
pub mod header_parser;
pub mod manager;
pub mod server_connection;
pub mod server_router;

use crate::error::Result;
use crate::utils::string_util::QueryExtractor;
use events::SocketEvents;
use server_router::ServerRouter;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::{net::TcpListener, sync::mpsc::UnboundedSender};
use tokio_tungstenite::tungstenite::http;
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

pub type SocketEventSender = UnboundedSender<SocketEvents>;

pub async fn start<V>(port: u16, router: Arc<ServerRouter>, token_validator: V) -> Result<()>
where
    V: Fn(&str) -> u32 + Send + Sync + 'static,
{
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!("WebSocket Server is running on ws://{}", addr);

    let (sender, receiver) = mpsc::unbounded_channel::<SocketEvents>();

    tokio::spawn(manager::start_loop(receiver));

    while let Ok((stream, client_addr)) = listener.accept().await {
        let mut id: u32 = 0;

        let callback = |req: &Request, mut res: Response| {
            if let Some(token) = req
                .uri()
                .query()
                .and_then(|query| query.extract_value("token").map(|t| t.to_string()))
            {
                id = token_validator(&token);
                if id == 0 {
                    *res.status_mut() = http::StatusCode::UNAUTHORIZED;
                }
            } else {
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
            }
            Ok(res)
        };

        match accept_hdr_async(stream, callback).await {
            Err(e) => println!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                println!("New client addr: {}", client_addr);
                tokio::spawn(server_connection::handle_connection(
                    router.clone(),
                    ws_stream,
                    sender.clone(),
                    id,
                ));
            }
        }
    }

    Ok(())
}
