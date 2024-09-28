extern crate core;

use std::io::Cursor;

use crate::error::ProtoCodecError;

mod endian;
pub use endian::*;

pub mod error;
pub mod types;

pub trait ProtoCodec: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;
}

pub trait GamePacket: Sized + ProtoCodec {
    const ID: u16;
    const COMPRESS: bool;
    const ENCRYPT: bool;

    #[inline]
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.proto_serialize(buf)
    }

    #[inline]
    fn deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Self::proto_deserialize(buf)
    }
}
