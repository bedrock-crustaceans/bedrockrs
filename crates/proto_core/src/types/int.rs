use std::io::Cursor;
use std::sync::Arc;

use crate::byteorder::ProtoCodecLE;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

impl ProtoCodec for u8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_u8(*self)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(stream.read_u8()?)
    }
}

impl ProtoCodec for i8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_i8(*self)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(stream.read_i8()?)
    }
}

macro_rules! impl_proto_codec_le {
    ($int:ident) => {
        impl ProtoCodecLE for $int {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                Ok(stream.(concat_idents!(write_, $int))::<LittleEndian>(*self)?)
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                Ok(stream.(concat_idents!(read_, $int))::<LittleEndian>()?)
            }
        }
    };
}

macro_rules! impl_proto_codec_be {
    ($int:ident) => {
        impl ProtoCodecBE for $int {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                Ok(stream.(concat_idents!(write_, $int))::<BigEndian>(*self)?)
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                Ok(stream.(concat_idents!(read_, $int))::<BigEndian>()?)
            }
        }
    };
}

macro_rules! impl_proto_codec_var {
    ($int:ident) => {
        impl ProtoCodecVar for $int {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                Ok(stream.(concat_idents!(write_, $int, _varint))(*self)?)
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                Ok(stream.(concat_idents!(read_, $int, _varint))()?)
            }
        }
    };
}

impl_proto_codec_le!(u16);
impl_proto_codec_le!(i16);
impl_proto_codec_le!(u32);
impl_proto_codec_le!(i32);
impl_proto_codec_le!(u64);
impl_proto_codec_le!(i64);
impl_proto_codec_le!(u128);
impl_proto_codec_le!(i128);
impl_proto_codec_le!(f32);
impl_proto_codec_le!(f64);

impl_proto_codec_be!(u16);
impl_proto_codec_be!(i16);
impl_proto_codec_be!(u32);
impl_proto_codec_be!(i32);
impl_proto_codec_be!(u64);
impl_proto_codec_be!(i64);
impl_proto_codec_be!(u128);
impl_proto_codec_be!(i128);
impl_proto_codec_be!(f32);
impl_proto_codec_be!(f64);

impl_proto_codec_var!(u16);
impl_proto_codec_var!(i16);
impl_proto_codec_var!(u32);
impl_proto_codec_var!(i32);
impl_proto_codec_var!(u64);
impl_proto_codec_var!(i64);
impl_proto_codec_var!(u128);
impl_proto_codec_var!(i128);
