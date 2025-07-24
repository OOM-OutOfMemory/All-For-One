use thiserror::Error;

#[derive(Error, Debug)]
pub enum AllForOneError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Session error: {0}")]
    MemcachedError(String),
    #[error("Logger error: {0}")]
    LoggerError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}
