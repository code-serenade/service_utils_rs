use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub jwt: JwtCfg,
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

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        c.try_deserialize()
    }
}
