use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElizaCliError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Command execution error: {0}")]
    CommandError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// A type alias for results returned by the CLI
pub type Result<T> = std::result::Result<T, ElizaCliError>;
