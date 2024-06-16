use std::io::{Cursor, Read, Write};

use bedrock_core::BE;

use crate::byte_order::NbtByteOrder;
use crate::error::NbtError;

pub struct NbtBigEndian;

impl NbtByteOrder for NbtBigEndian {
    #[inline]
    fn write_u8(buf: &mut Vec<u8>, byte: u8) -> Result<(), NbtError> {
        match BE::<u8>::write(&BE::new(byte), buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i16(buf: &mut Vec<u8>, int16: i16) -> Result<(), NbtError> {
        match BE::<i16>::write(&BE::new(int16), buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i32(buf: &mut Vec<u8>, int32: i32) -> Result<(), NbtError> {
        match BE::<i32>::write(&BE::new(int32), buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_i64(buf: &mut Vec<u8>, int64: i64) -> Result<(), NbtError> {
        match BE::<i64>::write(&BE::new(int64), buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_f32(buf: &mut Vec<u8>, float32: f32) -> Result<(), NbtError> {
        match BE::<f32>::write(&BE::new(float32), buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn write_f64(buf: &mut Vec<u8>, float64: f64) -> Result<(), NbtError> {
        match BE::<f64>::write(&BE::new(float64), buf) {
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
            Ok(_) => {}
            Err(e) => return Err(NbtError::IOError(e)),
        }

        Ok(())
    }

    #[inline]
    fn read_u8(buf: &mut Cursor<&[u8]>) -> Result<u8, NbtError> {
        match BE::<u8>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i16(buf: &mut Cursor<&[u8]>) -> Result<i16, NbtError> {
        match BE::<i16>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i32(buf: &mut Cursor<&[u8]>) -> Result<i32, NbtError> {
        match BE::<i32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_i64(buf: &mut Cursor<&[u8]>) -> Result<i64, NbtError> {
        match BE::<i64>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_f32(buf: &mut Cursor<&[u8]>) -> Result<f32, NbtError> {
        match BE::<f32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_f64(buf: &mut Cursor<&[u8]>) -> Result<f64, NbtError> {
        match BE::<f64>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(e)),
        }
    }

    #[inline]
    fn read_string(buf: &mut Cursor<&[u8]>) -> Result<String, NbtError> {
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
            Err(e) => return Err(NbtError::Utf8Error(e)),
        }
    }
}
