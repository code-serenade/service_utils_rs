use thiserror::Error;

#[cfg(feature = "jwt")]
use jsonwebtoken;

#[derive(Error, Debug)]
pub enum Error {
    #[error("auth error: {0}")]
    AuthError(String),

    #[cfg(feature = "jwt")]
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("{message:} ({line:}, {column})")]
    CustomError {
        message: String,
        line: u32,
        column: u32,
    },
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
