use std::borrow::Cow;

use thiserror::Error;

use crate::FieldType;

#[derive(Debug, Clone, Error)]
pub enum NbtError {
    #[error("An unknown tag type was encountered ({0}), it should be in the range 0-12")]
    TypeOutOfRange(u8),
    #[error("Expected tag of type {expected:?}, received {actual:?}")]
    UnexpectedType {
        expected: FieldType,
        actual: FieldType,
    },
    #[error("{0}")]
    Unsupported(&'static str),
    #[error("{0}")]
    MissingData(Cow<'static, str>),
    #[error("{0}")]
    ByteError(StreamError),
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

impl From<StreamError> for NbtError {
    fn from(value: StreamError) -> Self {
        Self::ByteError(value)
    }
}

#[derive(Debug, Clone, Error)]
pub enum StreamError {
    // TODO: std::io::Error does not implement Clone while the ProtoCodec error type requires it.
    // This is why I convert the error to a string rather than storing it directly like the others.
    #[error("{0}")]
    IoError(String),
    #[error("{0}")]
    FromUtf8Error(std::string::FromUtf8Error),
    #[error("{0}")]
    Utf8Error(std::str::Utf8Error),
    #[error("Expected {expected} remaining bytes, found only {remaining}")]
    UnexpectedEof { expected: usize, remaining: usize },
    #[error("{0}")]
    Other(Cow<'static, str>),
}

impl From<std::io::Error> for StreamError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<std::str::Utf8Error> for StreamError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<std::string::FromUtf8Error> for StreamError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
