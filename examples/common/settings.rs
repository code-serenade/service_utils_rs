use serde::Deserialize;
#[cfg(feature = "db")]
use service_utils_rs::services::db::SurrealdbCfg;
#[cfg(feature = "jwt")]
use service_utils_rs::services::jwt::JwtCfg;
use service_utils_rs::{error::Result, utils::load_settings};

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[cfg(feature = "jwt")]
    pub jwt: JwtCfg,
    #[cfg(feature = "db")]
    pub surrealdb: SurrealdbCfg,
    #[cfg(feature = "websocket")]
    pub websocket: WebsocketCfg,
    #[cfg(feature = "http")]
    pub http: HttpCfg,
}

/// Struct representing the websocket server configuration parameters.
#[cfg(feature = "websocket")]
#[derive(Debug, Deserialize)]
pub struct WebsocketCfg {
    pub port: u16,
}

/// Struct representing the http server configuration parameters.
#[cfg(feature = "http")]
#[derive(Debug, Deserialize)]
pub struct HttpCfg {
    pub port: u16,
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self> {
        load_settings(config_path)
    }
}
