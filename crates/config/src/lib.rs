use thiserror::Error;
#[derive(Debug, Error)] pub enum Error { #[error("missing environment variable: {0}")] Missing(String), #[error("invalid environment variable {0}")] Invalid(String) }
#[derive(Debug, Clone)] pub struct Config { pub bind: String, pub max_body_bytes: usize }
impl Config { pub fn from_env() -> Result<Self, Error> { let bind = std::env::var("WEBSITE_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into()); let max_body_bytes = std::env::var("WEBSITE_MAX_BODY_BYTES").unwrap_or_else(|_| "1048576".into()).parse().map_err(|_| Error::Invalid("WEBSITE_MAX_BODY_BYTES".into()))?; Ok(Self { bind, max_body_bytes }) } }
