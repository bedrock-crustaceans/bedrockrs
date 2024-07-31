use std::io::Error as IOError;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use std::sync::Arc;

use base64::DecodeError as Base64DecodeError;
use bedrockrs_nbt::error::NbtError;
use jsonwebtoken::errors::Error as JwtError;
use serde_json::error::Error as JsonError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ProtoCodecError {
    #[error("IOError occurred: {0}")]
    IOError(#[from] Arc<IOError>),
    #[error("NbtError: {0}")]
    NbtError(#[from] NbtError),
    #[error("Error while reading UTF8 encoded String: {0}")]
    UTF8Error(#[from] FromUtf8Error),
    #[error("Error while converting integers: {0}")]
    FromIntError(#[from] TryFromIntError),
    #[error("Json Error: {0}")]
    JsonError(#[from] Arc<JsonError>),
    #[error("Jwt Error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Base64 decoding Error: {0}")]
    Base64DecodeError(#[from] Base64DecodeError),
    #[error(
        "parse value `{0}` to enum variant for {1} enum"
    )]
    InvalidEnumID(String, String),
    #[error("Got an unknown/invalid game packet id: {0}")]
    InvalidGamePacketID(u16),
    #[error("Expected format got mismatched: {0}")]
    FormatMismatch(String),
}
