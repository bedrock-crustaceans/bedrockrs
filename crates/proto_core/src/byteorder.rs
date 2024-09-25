use std::io::Cursor;
use crate::error::ProtoCodecError;

pub trait ProtoCodecLE: Sized {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;
}

pub trait ProtoCodecBE: Sized {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;
}

pub trait ProtoCodecVAR: Sized {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;
}
