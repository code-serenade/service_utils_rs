use crate::error::{Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Represents the JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    /// Creates a new `Claims` instance.
    pub fn new(aud: String, sub: String, exp: usize, iat: usize) -> Self {
        Claims { aud, sub, exp, iat }
    }
}

/// Enum representing the type of token: ACCESS or REFRESH.
enum TokenKind {
    ACCESS,
    REFRESH,
}

impl Jwt {
    /// Generates a pair of access and refresh tokens.
    ///
    /// # Arguments
    ///
    /// * `sub` - A string slice that holds the subject.
    ///
    /// # Returns
    ///
    /// * A tuple containing the access token and the refresh token.
    pub fn generate_token_pair(&self, sub: String) -> (String, String) {
        let access_token = self.generate_token(TokenKind::ACCESS, sub.clone());
        let refresh_token = self.generate_token(TokenKind::REFRESH, sub);
        (access_token, refresh_token)
    }

    /// Generates an access token.
    ///
    /// # Arguments
    ///
    /// * `sub` - A string slice that holds the subject.
    ///
    /// # Returns
    ///
    /// * The generated access token as a string.
    pub fn generate_access_token(&self, sub: String) -> String {
        self.generate_token(TokenKind::ACCESS, sub)
    }

    /// Validates an access token.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the access token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the `Claims` if validation is successful, or an `Error` otherwise.
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, Error> {
        let result = self.validate_token(TokenKind::ACCESS, token)?;
        Ok(result.claims)
    }

    /// Validates a refresh token.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the refresh token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the `Claims` if validation is successful, or an `Error` otherwise.
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, Error> {
        let result = self.validate_token(TokenKind::REFRESH, token)?;
        Ok(result.claims)
    }

    /// Generates a token based on the token kind and subject.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of token (ACCESS or REFRESH).
    /// * `sub` - A string slice that holds the subject.
    ///
    /// # Returns
    ///
    /// * The generated token as a string.
    fn generate_token(&self, kind: TokenKind, sub: String) -> String {
        let aud = self.aud.clone();
        let duration = match kind {
            TokenKind::ACCESS => self.access_token_duration,
            TokenKind::REFRESH => self.refresh_token_duration,
        };
        let (iat, exp) = generate_expired_time(duration);
        let key = match kind {
            TokenKind::ACCESS => &self.encoding_access_key,
            TokenKind::REFRESH => &self.encoding_refresh_key,
        };
        let claims = Claims::new(aud, sub, exp, iat);
        encode(&self.header, &claims, key).unwrap() // Handle the error properly
    }

    /// Validates a token based on the token kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of token (ACCESS or REFRESH).
    /// * `token` - A string slice that holds the token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing `TokenData<Claims>` if validation is successful, or an `Error` otherwise.
    fn validate_token(&self, kind: TokenKind, token: &str) -> Result<TokenData<Claims>> {
        let (key, validation) = match kind {
            TokenKind::ACCESS => (&self.decoding_access_key, &self.validation_access_key),
            TokenKind::REFRESH => (&self.decoding_refresh_key, &self.validation_refresh_key),
        };
        Ok(decode::<Claims>(token, key, validation)?)
    }
}

/// Generates the issued at (iat) and expiration (exp) times based on the duration.
///
/// # Arguments
///
/// * `duration` - The duration in seconds for which the token is valid.
///
/// # Returns
///
/// * A tuple containing the issued at time and expiration time as UNIX timestamps.
fn generate_expired_time(duration: usize) -> (usize, usize) {
    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::seconds(duration as i64)).timestamp() as usize;
    (iat, exp)
}

/// Struct representing the JWT configuration.
#[derive(Clone)]
pub struct Jwt {
    header: Header,
    encoding_access_key: EncodingKey,
    encoding_refresh_key: EncodingKey,
    decoding_access_key: DecodingKey,
    decoding_refresh_key: DecodingKey,
    validation_access_key: Validation,
    validation_refresh_key: Validation,
    aud: String,
    access_token_duration: usize,
    refresh_token_duration: usize,
}

impl Jwt {
    /// Creates a new `Jwt` instance from the given configuration.
    ///
    /// # Arguments
    ///
    /// * `cfg` - A `JwtCfg` struct containing the JWT configuration.
    ///
    /// # Returns
    ///
    /// * A new `Jwt` instance.
    pub fn new(cfg: JwtCfg) -> Self {
        let header = Header::default();
        let encoding_access_key = EncodingKey::from_secret(cfg.access_secret.as_bytes());
        let encoding_refresh_key = EncodingKey::from_secret(cfg.refresh_secret.as_bytes());
        let decoding_access_key = DecodingKey::from_secret(cfg.access_secret.as_bytes());
        let decoding_refresh_key = DecodingKey::from_secret(cfg.refresh_secret.as_bytes());
        let mut validation_access_key = Validation::default();
        validation_access_key.set_audience(&[cfg.audience.clone()]);
        let mut validation_refresh_key = validation_access_key.clone();
        validation_refresh_key.validate_exp = false;
        validation_refresh_key.required_spec_claims.clear();
        let aud = cfg.audience;
        let access_token_duration = cfg.access_token_duration;
        let refresh_token_duration = cfg.refresh_token_duration;
        Jwt {
            header,
            encoding_access_key,
            encoding_refresh_key,
            decoding_access_key,
            decoding_refresh_key,
            validation_access_key,
            validation_refresh_key,
            aud,
            access_token_duration,
            refresh_token_duration,
        }
    }
}

/// Struct representing the JWT configuration parameters.
#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Sets up a `Jwt` instance for testing.
    ///
    /// # Returns
    ///
    /// * A `Jwt` instance with test configuration.
    fn setup_jwt() -> Jwt {
        let cfg = JwtCfg {
            access_secret: "access_secret".to_string(),
            refresh_secret: "refresh_secret".to_string(),
            audience: "test_audience".to_string(),
            access_token_duration: 3600,   // 1 hour
            refresh_token_duration: 86400, // 1 day
        };
        Jwt::new(cfg)
    }

    #[test]
    fn test_generate_token_pair() {
        let jwt = setup_jwt();
        let (access_token, refresh_token) = jwt.generate_token_pair("test_sub".to_string());

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
    }

    #[test]
    fn test_generate_access_token() {
        let jwt = setup_jwt();
        let access_token = jwt.generate_access_token("test_sub".to_string());

        assert!(!access_token.is_empty());
    }

    #[test]
    fn test_validate_access_token() {
        let jwt = setup_jwt();
        let access_token = jwt.generate_access_token("test_sub".to_string());
        let validation_result = jwt.validate_access_token(&access_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_validate_refresh_token() {
        let jwt = setup_jwt();
        let (_, refresh_token) = jwt.generate_token_pair("test_sub".to_string());
        let validation_result = jwt.validate_refresh_token(&refresh_token);

        assert!(validation_result.is_ok());
        let claims = validation_result.unwrap();
        assert_eq!(claims.aud, "test_audience");
        assert_eq!(claims.sub, "test_sub");
    }

    #[test]
    fn test_expired_access_token() {
        use std::time::Duration as StdDuration;

        let jwt = setup_jwt();
        // Manually generate an expired token
        let iat = (SystemTime::now() - StdDuration::from_secs(7200))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let exp = (SystemTime::now() - StdDuration::from_secs(3600))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let claims = Claims::new(
            "test_audience".to_string(),
            "test_sub".to_string(),
            exp,
            iat,
        );
        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("access_secret".as_ref()),
        )
        .unwrap();

        let validation_result = jwt.validate_access_token(&access_token);

        assert!(validation_result.is_err());
        match validation_result.unwrap_err() {
            Error::JwtError(_) => (),
            _ => panic!("Expected ErrorKind::ExpiredSignature"),
        }
    }

    #[test]
    fn test_invalid_access_token() {
        let jwt = setup_jwt();
        let invalid_token = "invalid_token";

        let validation_result = jwt.validate_access_token(invalid_token);

        assert!(validation_result.is_err());
        match validation_result.unwrap_err() {
            Error::JwtError(_) => (),
            _ => panic!("Expected ErrorKind::InvalidToken"),
        }
    }
}
