use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::int::{BE, LE, VAR};
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for u8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        stream
            .write_u8(*self)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        stream
            .read_u8()
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }
}

impl ProtoCodec for i8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        stream
            .write_i8(*self)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        stream
            .read_i8()
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }
}

macro_rules! impl_proto_codec {
    ($wrapper:ident, $int:ty) => {
        impl ProtoCodec for $wrapper<$int> {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                $wrapper::<$int>::write(self, stream)
                    .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                $wrapper::<$int>::read(stream).map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
            }
        }
    };
}

impl_proto_codec!(LE, u16);
impl_proto_codec!(LE, i16);
impl_proto_codec!(LE, u32);
impl_proto_codec!(LE, i32);
impl_proto_codec!(LE, u64);
impl_proto_codec!(LE, i64);
impl_proto_codec!(LE, u128);
impl_proto_codec!(LE, i128);
impl_proto_codec!(LE, f32);
impl_proto_codec!(LE, f64);

impl_proto_codec!(BE, u16);
impl_proto_codec!(BE, i16);
impl_proto_codec!(BE, u32);
impl_proto_codec!(BE, i32);
impl_proto_codec!(BE, u64);
impl_proto_codec!(BE, i64);
impl_proto_codec!(BE, u128);
impl_proto_codec!(BE, i128);
impl_proto_codec!(BE, f32);
impl_proto_codec!(BE, f64);

impl_proto_codec!(VAR, u16);
impl_proto_codec!(VAR, i16);
impl_proto_codec!(VAR, u32);
impl_proto_codec!(VAR, i32);
impl_proto_codec!(VAR, u64);
impl_proto_codec!(VAR, i64);
impl_proto_codec!(VAR, u128);
impl_proto_codec!(VAR, i128);
