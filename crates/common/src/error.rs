use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Timeout")]
    Timeout,
    #[error("Protocol error")]
    ProtocolError,
    #[error("Connection failed")]
    ConnectionFailed,
}
