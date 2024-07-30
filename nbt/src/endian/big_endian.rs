use std::io::{Cursor, Read, Write};
use std::sync::Arc;
use bedrockrs_core::int::be::BE;
use crate::byte_order::NbtByteOrder;
use crate::error::NbtError;

pub struct NbtBigEndian;

impl NbtByteOrder for NbtBigEndian {
    #[inline]
    fn write_u8(buf: &mut Vec<u8>, byte: u8) -> Result<(), NbtError> {
        BE::<u8>::write(&BE::new(byte), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i16(buf: &mut Vec<u8>, int16: i16) -> Result<(), NbtError> {
        BE::<i16>::write(&BE::new(int16), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i32(buf: &mut Vec<u8>, int32: i32) -> Result<(), NbtError> {
        BE::<i32>::write(&BE::new(int32), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i64(buf: &mut Vec<u8>, int64: i64) -> Result<(), NbtError> {
        BE::<i64>::write(&BE::new(int64), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_f32(buf: &mut Vec<u8>, float32: f32) -> Result<(), NbtError> {
        BE::<f32>::write(&BE::new(float32), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_f64(buf: &mut Vec<u8>, float64: f64) -> Result<(), NbtError> {
        BE::<f64>::write(&BE::new(float64), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_string(buf: &mut Vec<u8>, string: String) -> Result<(), NbtError> {
        Self::write_i16(
            buf,
            string.len().try_into().map_err(|e| NbtError::IntError(e))?,
        )?;

        buf.write_all(string.as_bytes())
            .map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn read_u8(buf: &mut Cursor<&[u8]>) -> Result<u8, NbtError> {
        match BE::<u8>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i16(buf: &mut Cursor<&[u8]>) -> Result<i16, NbtError> {
        match BE::<i16>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i32(buf: &mut Cursor<&[u8]>) -> Result<i32, NbtError> {
        match BE::<i32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i64(buf: &mut Cursor<&[u8]>) -> Result<i64, NbtError> {
        match BE::<i64>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_f32(buf: &mut Cursor<&[u8]>) -> Result<f32, NbtError> {
        match BE::<f32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_f64(buf: &mut Cursor<&[u8]>) -> Result<f64, NbtError> {
        match BE::<f64>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_string(buf: &mut Cursor<&[u8]>) -> Result<String, NbtError> {
        let len = Self::read_i16(buf)?;

        let mut string_buf = vec![0; len.try_into().map_err(|e| NbtError::IntError(e))?];

        buf.read_exact(&mut string_buf)
            .map_err(|e| NbtError::IOError(Arc::new(e)))?;

        String::from_utf8(string_buf).map_err(|e| NbtError::Utf8Error(e))
    }
}
