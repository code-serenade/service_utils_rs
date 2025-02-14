use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub struct Router {
    handlers: HashMap<String, Arc<dyn Fn(serde_json::Value) + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            handlers: HashMap::new(),
        }
    }

    // 注册处理函数
    pub fn add<F>(&mut self, route: String, handler: F)
    where
        F: Fn(serde_json::Value) + Send + Sync + 'static,
    {
        self.handlers.insert(route, Arc::new(handler));
    }

    // 通过 action 字段进行路由处理
    pub fn handle(&self, action: &str, data: serde_json::Value) {
        if let Some(handler) = self.handlers.get(action) {
            handler(data);
        } else {
            unknown_handler(action, data);
        }
    }
}

fn unknown_handler(action: &str, data: serde_json::Value) {
    println!("Unknown action: {}", action);
    println!("Data: {}", data);
}

impl Clone for Router {
    fn clone(&self) -> Self {
        let handlers = self
            .handlers
            .iter()
            .map(|(route, handler)| (route.clone(), Arc::clone(handler)))
            .collect::<HashMap<String, Arc<dyn Fn(serde_json::Value) + Send + Sync>>>();

        Router { handlers }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub action: String,
    pub data: serde_json::Value,
}
