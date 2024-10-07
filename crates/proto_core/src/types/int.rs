use crate::endian::{ProtoCodecBE, ProtoCodecLE, ProtoCodecVAR};
use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use paste::paste;
use std::io::Cursor;
use varint_rs::VarintReader;
use varint_rs::VarintWriter;

impl ProtoCodec for u8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_u8(*self)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(stream.read_u8()?)
    }

    fn get_size_prediction(&self) -> usize {
        1
    }
}

impl ProtoCodec for i8 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_i8(*self)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(stream.read_i8()?)
    }

    fn get_size_prediction(&self) -> usize {
        1
    }
}

macro_rules! impl_proto_codec_le {
    ($int:ident, $size:literal) => {
        paste! {
            impl ProtoCodecLE for $int {
                fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                    Ok(WriteBytesExt::[<write_ $int>]::<LittleEndian>(stream, *self)?)
                }

                fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                    Ok(ReadBytesExt::[<read_ $int>]::<LittleEndian>(stream)?)
                }
                
                fn get_size_prediction(&self) -> usize {
                    $size
                }
            }
        }
    };
}

macro_rules! impl_proto_codec_be {
    ($int:ident, $size:literal) => {
        paste! {
            impl ProtoCodecBE for $int {
                fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                    Ok(WriteBytesExt::[<write_ $int>]::<BigEndian>(stream, *self)?)
                }

                fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                    Ok(ReadBytesExt::[<read_ $int>]::<BigEndian>(stream)?)
                }
                
                fn get_size_prediction(&self) -> usize {
                    $size
                }
            }
        }
    };
}

macro_rules! impl_proto_codec_var {
    ($int:ident, $size:literal) => {
        paste! {
            impl ProtoCodecVAR for $int {
                fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                    Ok(VarintWriter::[<write_ $int _varint>](stream, *self)?)
                }

                fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                    Ok(VarintReader::[<read_ $int _varint>](stream)?)
                }
                
                fn get_size_prediction(&self) -> usize {
                    $size
                }
            }
        }
    };
}

impl_proto_codec_le!(u16, 2);
impl_proto_codec_le!(i16, 2);
impl_proto_codec_le!(u32, 4);
impl_proto_codec_le!(i32, 4);
impl_proto_codec_le!(u64, 8);
impl_proto_codec_le!(i64, 8);
impl_proto_codec_le!(u128, 16);
impl_proto_codec_le!(i128, 16);
impl_proto_codec_le!(f32, 4);
impl_proto_codec_le!(f64, 8);

impl_proto_codec_be!(u16, 2);
impl_proto_codec_be!(i16, 2);
impl_proto_codec_be!(u32, 4);
impl_proto_codec_be!(i32, 4);
impl_proto_codec_be!(u64, 8);
impl_proto_codec_be!(i64, 8);
impl_proto_codec_be!(u128, 16);
impl_proto_codec_be!(i128, 16);
impl_proto_codec_be!(f32, 4);
impl_proto_codec_be!(f64, 8);

impl_proto_codec_var!(u16, 2);
impl_proto_codec_var!(i16, 2);
impl_proto_codec_var!(u32, 4);
impl_proto_codec_var!(i32, 4);
impl_proto_codec_var!(u64, 8);
impl_proto_codec_var!(i64, 8);
impl_proto_codec_var!(u128, 16);
impl_proto_codec_var!(i128, 16);
