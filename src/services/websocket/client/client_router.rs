use crate::services::websocket::JsonMessage;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait Handler {
    fn call(
        &self,
        data: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>>;
}

impl<F, Fut> Handler for F
where
    F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<JsonMessage>> + Send + 'static,
{
    fn call(
        &self,
        data: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>> {
        Box::pin((self)(data))
    }
}

pub struct ClientRouter {
    routes: HashMap<&'static str, Arc<dyn Handler + Send + Sync>>,
}

impl ClientRouter {
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
    ) -> Option<JsonMessage> {
        if let Some(handler) = self.routes.get(action) {
            handler.call(data).await
        } else {
            eprintln!("Unknown action: {}", action);
            None
        }
    }
}
