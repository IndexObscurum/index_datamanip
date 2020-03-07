//! Items useful for handling errors

/// Holds the result of various operations in this library
pub type Result<T> = std::result::Result<T, Error>;

/// The `Error` enum is used to represent various forms of failure
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error related to an IO operation, such as reading the file from permanent
    /// storage
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    /// The data was unable to be parsed into the expected structures or
    /// attempted to reference other data which didn't exist.
    #[error("failed to parse: {0}")]
    ParseError(String),
    /// The requested item wasn't found
    #[error("item not found in collection: {0}")]
    ItemNotFound(String),
    /// The bytes weren't valid utf8 data. Unfortunately, CoH doesn't use utf8.
    #[error("encoding error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::ParseError(msg.to_string())
    }
}

impl std::convert::From<nom::Err<(&[u8], nom::error::ErrorKind)>> for Error {
    fn from(err: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
        Error::ParseError(format!("{}", err))
    }
}
