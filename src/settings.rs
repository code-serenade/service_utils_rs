use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub jwt: JwtCfg,
    #[cfg(feature = "db")]
    pub surrealdb: SurrealdbCfg,
    #[cfg(feature = "websocket")]
    pub websocket: WebsocketCfg,
    pub http: HttpCfg,
}

/// Struct representing the JWT configuration parameters.
#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
    pub access_key_validate_exp: bool,
    pub refresh_key_validate_exp: bool,
}

/// Struct representing the Surrealdb configuration parameters.
#[cfg(feature = "db")]
#[derive(Debug, Deserialize)]
pub struct SurrealdbCfg {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

/// Struct representing the websocket server configuration parameters.
#[cfg(feature = "websocket")]
#[derive(Debug, Deserialize)]
pub struct WebsocketCfg {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct HttpCfg {
    pub port: u16,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        c.try_deserialize()
    }
}
