use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NbtError {
    #[error("Unexpectedly reached end of buffer")]
    UnexpectedEOF,
    #[error("Unexpected tag id {0} while reading")]
    UnexpectedID(u8),
    #[error("Error while reading UTF8 encoded String: {0}")]
    Utf8Error(#[from] FromUtf8Error ),
    #[error("Error while converting integers: {0}")]
    IntError(#[from] TryFromIntError),
    #[error("Got unexpetcted Compound Closing Tag")]
    CompoundClosingTag,
    #[error("IOError while reading/writing: {0}")]
    IOError(String),
}