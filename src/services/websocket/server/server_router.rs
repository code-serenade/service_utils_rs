use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use super::SocketEventSender;
use crate::services::websocket::JsonMessage;

pub trait Handler {
    fn call(
        &self,
        data: serde_json::Value,
        tx: SocketEventSender,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>>;
}

impl<F, Fut> Handler for F
where
    F: Fn(serde_json::Value, SocketEventSender) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<JsonMessage>> + Send + 'static,
{
    fn call(
        &self,
        data: serde_json::Value,
        tx: SocketEventSender,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>> {
        Box::pin((self)(data, tx))
    }
}

pub struct ServerRouter {
    routes: HashMap<&'static str, Arc<dyn Handler + Send + Sync>>,
}

impl ServerRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<H>(&mut self, action: &'static str, handler: H) -> &mut Self
    where
        H: Handler + Send + Sync + 'static,
    {
        self.routes.insert(action, Arc::new(handler));
        self
    }

    pub async fn handle_message(
        &self,
        action: &str,
        data: serde_json::Value,
        tx: SocketEventSender,
    ) -> Option<JsonMessage> {
        if let Some(handler) = self.routes.get(action) {
            handler.call(data, tx).await
        } else {
            eprintln!("Unknown action: {}", action);
            None
        }
    }
}
