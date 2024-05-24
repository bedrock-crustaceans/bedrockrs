use std::io::Error as IOError;
use std::string::FromUtf8Error;

use base64::DecodeError as Base64DecodeError;
use jsonwebtoken::errors::Error as JwtError;
use nbt::error::NbtError;
use serde_json::error::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtoCodecError {
    #[error("IOError occurred: {0}")]
    IOError(#[from] IOError),
    #[error("NbtError: {0}")]
    NbtError(#[from] NbtError),
    #[error("Error while reading UTF8 encoded String: {0}")]
    UTF8Error(#[from] FromUtf8Error),
    #[error("Json Error: {0}")]
    JsonError(#[from] JsonError),
    #[error("Jwt Error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Base64 decoding Error: {0}")]
    Base64DecodeError(#[from] Base64DecodeError),
    #[error("Could not convert enum variant to integer or parse integer to enum variant")]
    InvalidEnumID,
    #[error("Got an unknown/invalid game packet id: {0}")]
    InvalidGamePacketID(u16),
    #[error("Expected format got mismatched: {0}")]
    FormatMismatch(String),
}
