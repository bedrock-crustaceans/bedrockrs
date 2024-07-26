extern crate core;

use std::io::Cursor;

use crate::error::ProtoCodecError;

pub mod error;
pub mod types;

pub trait ProtoCodec: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;
}
