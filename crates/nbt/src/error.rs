use std::borrow::Cow;

use thiserror::Error;

use crate::FieldType;

/// Errors that can occur while serializing or deserializing NBT data.
#[derive(Debug, Clone, Error)]
pub enum NbtError {
    /// The encountered NBT tag type is invalid.
    #[error("An unknown tag type was encountered ({0}), it should be in the range 0-12")]
    TypeOutOfRange(u8),
    /// Found a type different from the type that was expected.
    #[error("Expected tag of type {expected:?}, received {actual:?}")]
    UnexpectedType {
        /// Type that the deserializer was expecting to find.
        expected: FieldType,
        /// Type that was found in the NBT stream.
        actual: FieldType,
    },
    /// The requested operation is not supported.
    #[error("{0}")]
    Unsupported(&'static str),
    /// Any errors related to reading and writing from the stream.
    #[error("{0}")]
    ByteError(#[from] StreamError),
    /// Other errors that do not fit in any of the previous categories.
    #[error("{0}")]
    Other(Cow<'static, str>),
}

impl From<std::io::Error> for NbtError {
    fn from(value: std::io::Error) -> Self {
        Self::ByteError(StreamError::IoError(value.to_string()))
    }
}

impl From<std::str::Utf8Error> for NbtError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::ByteError(StreamError::Utf8Error(value))
    }
}

impl From<std::string::FromUtf8Error> for NbtError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::ByteError(StreamError::FromUtf8Error(value))
    }
}

/// Errors related to binary reading and writing.
#[derive(Debug, Clone, Error)]
pub enum StreamError {
    // TODO: std::io::Error does not implement Clone while the ProtoCodec error type requires it.
    // This is why I convert the error to a string rather than storing it directly like the others.
    /// An IO [`Error`](std::io::Error).
    #[error("{0}")]
    IoError(String),
    /// A byte slice could not be converted into a `String` because it is invalid UTF-8.
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// A byte slice could not be converted into a `str` because it is invalid UTF-8.
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// The deserializer tried to read past the end of the buffer.
    #[error("Expected {expected} remaining bytes, found only {remaining}")]
    UnexpectedEof { expected: usize, remaining: usize },
    /// Any errors that do not fit the previous categories.
    #[error("{0}")]
    Other(Cow<'static, str>),
}

impl From<std::io::Error> for StreamError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}
