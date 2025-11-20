use thiserror::Error;

/// Errors returned by the parser
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid format in line: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Duplicate field detected: {0}")]
    DuplicateField(String),

    #[error("Unknown rule: {0}")]
    UnknownRule(String),

    #[error("Invalid number in line: {0}")]
    InvalidNumber(String),

    #[error("Empty value for field: {0}")]
    EmptyValue(String),
}
