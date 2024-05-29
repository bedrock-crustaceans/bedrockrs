use std::str::FromStr;
use crate::{u128be, u128le};

/// A Universally Unique Identifier (UUID).
/// (A simple wrapper around the uuid crates uuid::Uuid type)
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UUID(uuid::Uuid);

/// A general error that can occur when working with UUIDs.
pub struct UUIDError(pub uuid::Error);

/// Parse Uuids from string literals at compile time.
/// This macro transforms the string literal representation of an Uuid into the bytes
/// representation, raising a compilation error if it cannot properly be parsed.
#[macro_export]
macro_rules! uuid {
    ($uuid:literal) => {
        uuid::uuid!(uuid)
    };
}

impl FromStr for UUID {
    type Err = UUIDError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match uuid::Uuid::from_str(s) {
            Ok(v) => { Ok(Self(v)) }
            Err(e) => { Err(UUIDError(e)) }
        }
    }
}

impl UUID {
    /// Creates a random UUID.
    #[inline]
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Creates a UUID using the UUID crates Uuid type.
    #[inline]
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Creates a UUID using the supplied bytes.
    /// This function will return an error if b has any length other than 16.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Result<Self, UUIDError> {
        match uuid::Uuid::from_slice(slice) {
            Ok(v) => { Ok(Self(v)) }
            Err(e) => { Err(UUIDError(e)) }
        }
    }

    /// Creates a UUID from two 64bit values.
    #[inline]
    pub fn from_u64_pair(high_bits: u64, low_bits: u64) -> Self {
        Self(uuid::Uuid::from_u64_pair(high_bits, low_bits))
    }

    /// Creates a UUID from a 128bit value.
    #[inline]
    pub fn from_u128be(v: u128be) -> Self {
        Self(uuid::Uuid::from_u128(v.0))
    }

    /// Creates a UUID from a 128bit value.
    #[inline]
    pub fn from_u128le(v: u128le) -> Self {
        Self(uuid::Uuid::from_u128_le(v.0))
    }
}