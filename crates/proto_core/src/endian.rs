use crate::error::ProtoCodecError;
use std::io::Cursor;

pub trait ProtoCodecLE: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;

    fn get_size_prediction(&self) -> usize;
}

pub trait ProtoCodecBE: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;

    fn get_size_prediction(&self) -> usize;
}

pub trait ProtoCodecVAR: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;

    fn get_size_prediction(&self) -> usize;
}
