use bedrock_core::stream::read::ByteStreamRead;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use varint_rs::VarintWriter;

use crate::byte_order::NbtByteOrder;
use crate::error::NbtError;

pub struct NbtLittleEndianNetwork;

impl NbtByteOrder for NbtLittleEndianNetwork {
    #[inline]
    fn write_u8(buf: &mut Vec<u8>, byte: u8) -> Result<(), NbtError> {
        match buf.write_u8_varint(byte) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i16(buf: &mut Vec<u8>, int16: i16) -> Result<(), NbtError> {
        match buf.write_i16::<LittleEndian>(int16) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i32(buf: &mut Vec<u8>, int32: i32) -> Result<(), NbtError> {
        match buf.write_i32_varint(int32) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i64(buf: &mut Vec<u8>, int64: i64) -> Result<(), NbtError> {
        match buf.write_i64_varint(int64) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_f32(buf: &mut Vec<u8>, float32: f32) -> Result<(), NbtError> {
        match buf.write_f32::<LittleEndian>(float32) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_f64(buf: &mut Vec<u8>, float64: f64) -> Result<(), NbtError> {
        match buf.write_f64::<LittleEndian>(float64) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_string(buf: &mut Vec<u8>, string: String) -> Result<(), NbtError> {
        match Self::write_i16(
            buf,
            match string.len().try_into() {
                Ok(v) => v,
                Err(e) => return Err(NbtError::IntError(e)),
            },
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match buf.write_all(string.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_u8(buf: &mut ByteStreamRead) -> Result<u8, NbtError> {
        match buf.read_u8() {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i16(buf: &mut ByteStreamRead) -> Result<i16, NbtError> {
        match buf.read_i16le() {
            Ok(v) => Ok(v.0),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i32(buf: &mut ByteStreamRead) -> Result<i32, NbtError> {
        match buf.read_ivar32() {
            Ok(v) => Ok(v.0),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i64(buf: &mut ByteStreamRead) -> Result<i64, NbtError> {
        match buf.read_ivar64() {
            Ok(v) => Ok(v.0),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_f32(buf: &mut ByteStreamRead) -> Result<f32, NbtError> {
        match buf.read_f32le() {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_f64(buf: &mut ByteStreamRead) -> Result<f64, NbtError> {
        match buf.read_f64le() {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_string(buf: &mut ByteStreamRead) -> Result<String, NbtError> {
        let len = match Self::read_i16(buf) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let mut string_buf = vec![
            0;
            match len.try_into() {
                Ok(v) => {
                    v
                }
                Err(e) => {
                    return Err(NbtError::IntError(e));
                }
            }
        ];

        match buf.read_exact(&mut string_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(NbtError::IOError(e));
            }
        };

        match String::from_utf8(string_buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::Utf8Error(e)),
        }
    }
}
