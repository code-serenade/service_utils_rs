use config::{Config, ConfigError};
use serde::Deserialize;

#[cfg(feature = "jwt")]
use crate::services::jwt::JwtCfg;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[cfg(feature = "jwt")]
    pub jwt: JwtCfg,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name("config/services"))
            .build()?;
        c.try_deserialize()
    }
}
