use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("auth error: {0}")]
    AuthError(String),

    #[cfg(feature = "jwt")]
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[cfg(feature = "websocket")]
    #[error("websocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[cfg(feature = "http")]
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),

    #[cfg(feature = "db")]
    #[error("db error: {0}")]
    DbError(#[from] surrealdb::Error),

    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("{message:} ({line:}, {column})")]
    CustomError {
        message: String,
        line: u32,
        column: u32,
    },

    #[error("error message: {0}")]
    ErrorMessage(String),

    #[error("error code: {0}")]
    ErrorCode(u16),

    #[error("system error: {0}")]
    SystemError(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
