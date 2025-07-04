use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use crate::services::websocket::JsonMessage;

pub trait Handler {
    fn call(
        &self,
        data: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>>;
}

impl<F> Handler for F
where
    F: Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>>
        + Send
        + Sync
        + 'static,
{
    fn call(
        &self,
        data: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>> {
        (self)(data)
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

#[macro_export]
macro_rules! add_handler {
    ($router:expr, $action:expr, $typ:ty, $func:expr) => {{
        use std::{future::Future, pin::Pin};
        let wrapper = move |data: serde_json::Value| {
            let parsed: $typ = match serde_json::from_value(data) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("JSON parse error for action {}: {}", $action, e);
                    return Box::pin(async { None }) as Pin<Box<dyn Future<Output = _> + Send>>;
                }
            };

            let fut = async move { $func(parsed).await };
            Box::pin(fut) as Pin<Box<dyn Future<Output = Option<JsonMessage>> + Send>>
        };
        $router.add_route($action, wrapper);
    }};
}
