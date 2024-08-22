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
    #[error("{0} is not supported")]
    Unsupported(&'static str),
    #[error("{0}")]
    MissingData(Cow<'static, str>),
    #[error("{0}")]
    Other(&'static str),
}
