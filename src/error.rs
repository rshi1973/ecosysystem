use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialization error {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Custom error: {0}")]
    Custom(String),
}
