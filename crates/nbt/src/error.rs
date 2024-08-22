use std::borrow::Cow;

use thiserror::Error;

use crate::FieldType;

#[derive(Debug, Error)]
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
    Other(Cow<'static, str>),
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("{0}")]
    Utf8Error(std::str::Utf8Error),
    #[error("Expected {expected} remaining bytes, found only {remaining}")]
    UnexpectedEof { expected: usize, remaining: usize },
    #[error("{0}")]
    Other(Cow<'static, str>),
}

impl From<std::str::Utf8Error> for StreamError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}
