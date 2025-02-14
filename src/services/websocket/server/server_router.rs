use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::SocketEventSender;

pub struct ServerRouter {
    handlers: HashMap<String, Arc<dyn Fn(serde_json::Value, SocketEventSender) + Send + Sync>>,
}

impl ServerRouter {
    pub fn new() -> Self {
        ServerRouter {
            handlers: HashMap::new(),
        }
    }

    // 注册处理函数
    pub fn add<F>(&mut self, route: String, handler: F)
    where
        F: Fn(serde_json::Value, SocketEventSender) + Send + Sync + 'static,
    {
        self.handlers.insert(route, Arc::new(handler));
    }

    // 通过 action 字段进行路由处理
    pub fn handle(&self, action: &str, data: serde_json::Value, tx: SocketEventSender) {
        if let Some(handler) = self.handlers.get(action) {
            handler(data, tx);
        } else {
            unknown_handler(action, data);
        }
    }
}

fn unknown_handler(action: &str, data: serde_json::Value) {
    println!("Unknown action: {}", action);
    println!("Data: {}", data);
}

impl Clone for ServerRouter {
    fn clone(&self) -> Self {
        let handlers = self
            .handlers
            .iter()
            .map(|(route, handler)| (route.clone(), Arc::clone(handler)))
            .collect::<HashMap<String, Arc<dyn Fn(serde_json::Value, SocketEventSender) + Send + Sync>>>();

        ServerRouter { handlers }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub action: String,
    pub data: serde_json::Value,
}
