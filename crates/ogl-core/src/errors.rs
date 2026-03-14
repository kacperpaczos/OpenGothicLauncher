use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    #[error("Invalid state: {0}")]
    InvalidState(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("External error: {0}")]
    External(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),
}
