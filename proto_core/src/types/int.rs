use std::sync::Arc;

use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use bedrock_core::*;
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for u8 {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        match stream.write_u8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        match stream.read_u8() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }
}

impl ProtoCodec for i8 {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        match stream.write_i8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        match stream.read_i8() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }
}

macro_rules! impl_proto_codec {
    ($wrapper:ident, $int:ty) => {
        impl ProtoCodec for $wrapper<$int> {
            fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
                match $wrapper::<$int>::write(self, stream) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
                }
            }

            fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
                match $wrapper::<$int>::read(stream) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
                }
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
