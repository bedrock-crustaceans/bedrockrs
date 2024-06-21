use std::io::Error as IOError;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use std::sync::Arc;

use thiserror::Error;

/// All errors that the NBT implementation can output.
/// (Use `thiserror` for an easy implementation of the standard libraries
/// [Error]([std::error::Error]) trait)
#[derive(Error, Debug, Clone)]
pub enum NbtError {
    #[error("Unexpectedly reached end of buffer")]
    UnexpectedEOF,
    #[error("Unexpected tag id {0} while reading")]
    UnexpectedID(u8),
    #[error("Error while reading UTF8 encoded String: {0}")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Error while converting integers: {0}")]
    IntError(#[from] TryFromIntError),
    #[error("IO Error: {0}")]
    IOError(#[from] Arc<IOError>),
}
