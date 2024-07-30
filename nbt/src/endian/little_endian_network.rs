use std::io::{Cursor, Read, Write};
use std::sync::Arc;

use bedrockrs_core::int::{LE, VAR};

use crate::byte_order::NbtByteOrder;
use crate::error::NbtError;

pub struct NbtLittleEndianNetwork;

impl NbtByteOrder for NbtLittleEndianNetwork {
    #[inline]
    fn write_u8(buf: &mut Vec<u8>, byte: u8) -> Result<(), NbtError> {
        LE::<u8>::write(&LE::new(byte), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i16(buf: &mut Vec<u8>, int16: i16) -> Result<(), NbtError> {
        LE::<i16>::write(&LE::new(int16), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i32(buf: &mut Vec<u8>, int32: i32) -> Result<(), NbtError> {
        VAR::<i32>::write(&VAR::new(int32), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_i64(buf: &mut Vec<u8>, int64: i64) -> Result<(), NbtError> {
        VAR::<i64>::write(&VAR::new(int64), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_f32(buf: &mut Vec<u8>, float32: f32) -> Result<(), NbtError> {
        LE::<f32>::write(&LE::new(float32), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
    }

    #[inline]
    fn write_f64(buf: &mut Vec<u8>, float64: f64) -> Result<(), NbtError> {
        LE::<f64>::write(&LE::new(float64), buf).map_err(|e| NbtError::IOError(Arc::new(e)))
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
        match LE::<u8>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i16(buf: &mut Cursor<&[u8]>) -> Result<i16, NbtError> {
        match LE::<i16>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i32(buf: &mut Cursor<&[u8]>) -> Result<i32, NbtError> {
        match VAR::<i32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_i64(buf: &mut Cursor<&[u8]>) -> Result<i64, NbtError> {
        match VAR::<i64>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_f32(buf: &mut Cursor<&[u8]>) -> Result<f32, NbtError> {
        match LE::<f32>::read(buf) {
            Ok(v) => Ok(v.into_inner()),
            Err(e) => Err(NbtError::IOError(Arc::new(e))),
        }
    }

    #[inline]
    fn read_f64(buf: &mut Cursor<&[u8]>) -> Result<f64, NbtError> {
        match LE::<f64>::read(buf) {
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
