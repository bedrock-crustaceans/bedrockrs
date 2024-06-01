use std::io::Read;
use bytes::{Buf, Bytes};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use bytes::buf::Reader;
use varint_rs::VarintReader;
use crate::*;

/// A wrapper around [`bytes::Bytes`].
/// (A cheaply cloneable and sliceable chunk of contiguous memory).
#[repr(transparent)]
pub struct ByteStreamRead(Reader<Bytes>);

impl ByteStreamRead {
    /// Creates a new empty `ByteStreamRead`.
    /// This will not allocate and the returned Bytes handle will be empty.
    #[inline]
    pub fn new() -> Self {
        Self(Bytes::new().reader())
    }

    /// Creates a new empty `ByteStreamRead` from a given [`bytes::Bytes`] object.
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self(bytes.reader())
    }

    /// Creates a new `ByteStreamRead` from a static slice.
    #[inline]
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self(Bytes::from_static(bytes).reader())
    }

    /// Returns the number of bytes contained in this `ByteStreamRead`.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.get_ref().len()
    }

    /// Returns true if the `ByteStreamRead` has a length of 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.get_ref().is_empty()
    }

    /// Clears the buffer, removing all data. Existing capacity is preserved.
    #[inline]
    pub fn clear(&mut self) {
        self.0.get_mut().clear();
    }

    /// Read an u8
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        self.0.read_u8()
    }

    /// Read an i8
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8, std::io::Error> {
        self.0.read_i8()
    }

    /// Read an u16le
    #[inline]
    pub fn read_u16le(&mut self) -> Result<u16le, std::io::Error> {
        match self.0.read_u16::<LittleEndian>() {
            Ok(v) => { Ok(u16le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u16be
    #[inline]
    pub fn read_u16be(&mut self) -> Result<u16be, std::io::Error> {
        match self.0.read_u16::<BigEndian>() {
            Ok(v) => { Ok(u16be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i16le
    #[inline]
    pub fn read_i16le(&mut self) -> Result<i16le, std::io::Error> {
        match self.0.read_i16::<LittleEndian>() {
            Ok(v) => { Ok(i16le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i16be
    #[inline]
    pub fn read_i16be(&mut self) -> Result<i16be, std::io::Error> {
        match self.0.read_i16::<BigEndian>() {
            Ok(v) => { Ok(i16be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u32le
    #[inline]
    pub fn read_u32le(&mut self) -> Result<u32le, std::io::Error> {
        match self.0.read_u32::<LittleEndian>() {
            Ok(v) => { Ok(u32le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u32be
    #[inline]
    pub fn read_u32be(&mut self) -> Result<u32be, std::io::Error> {
        match self.0.read_u32::<BigEndian>() {
            Ok(v) => { Ok(u32be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i32le
    #[inline]
    pub fn read_i32le(&mut self) -> Result<i32le, std::io::Error> {
        match self.0.read_i32::<LittleEndian>() {
            Ok(v) => { Ok(i32le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i32be
    #[inline]
    pub fn read_i32be(&mut self) -> Result<i32be, std::io::Error> {
        match self.0.read_i32::<BigEndian>() {
            Ok(v) => { Ok(i32be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u64le
    #[inline]
    pub fn read_u64le(&mut self) -> Result<u64le, std::io::Error> {
        match self.0.read_u64::<LittleEndian>() {
            Ok(v) => { Ok(u64le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u64be
    #[inline]
    pub fn read_u64be(&mut self) -> Result<u64be, std::io::Error> {
        match self.0.read_u64::<BigEndian>() {
            Ok(v) => { Ok(u64be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i64le
    #[inline]
    pub fn read_i64le(&mut self) -> Result<i64le, std::io::Error> {
        match self.0.read_i64::<LittleEndian>() {
            Ok(v) => { Ok(i64le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i64be
    #[inline]
    pub fn read_i64be(&mut self) -> Result<i64be, std::io::Error> {
        match self.0.read_i64::<BigEndian>() {
            Ok(v) => { Ok(i64be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u128le
    #[inline]
    pub fn read_u128le(&mut self) -> Result<u128le, std::io::Error> {
        match self.0.read_u128::<LittleEndian>() {
            Ok(v) => { Ok(u128le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an u128be
    #[inline]
    pub fn read_u128be(&mut self) -> Result<u128be, std::io::Error> {
        match self.0.read_u128::<BigEndian>() {
            Ok(v) => { Ok(u128be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i128le
    #[inline]
    pub fn read_i128le(&mut self) -> Result<i128le, std::io::Error> {
        match self.0.read_i128::<LittleEndian>() {
            Ok(v) => { Ok(i128le(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an i128be
    #[inline]
    pub fn read_i128be(&mut self) -> Result<i128be, std::io::Error> {
        match self.0.read_i128::<BigEndian>() {
            Ok(v) => { Ok(i128be(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an uvar32
    #[inline]
    pub fn read_uvar32(&mut self) -> Result<uvar32, std::io::Error> {
        match self.0.read_u32_varint() {
            Ok(v) => { Ok(uvar32(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an ivar32
    #[inline]
    pub fn read_ivar32(&mut self) -> Result<ivar32, std::io::Error> {
        match self.0.read_i32_varint() {
            Ok(v) => { Ok(ivar32(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an uvar64
    #[inline]
    pub fn read_uvar64(&mut self) -> Result<uvar64, std::io::Error> {
        match self.0.read_u64_varint() {
            Ok(v) => { Ok(uvar64(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read an ivar64
    #[inline]
    pub fn read_ivar64(&mut self) -> Result<ivar64, std::io::Error> {
        match self.0.read_i64_varint() {
            Ok(v) => { Ok(ivar64(v)) }
            Err(e) => { Err(e) }
        }
    }

    /// Read a f32le
    #[inline]
    pub fn read_f32le(&mut self) -> Result<f32, std::io::Error> {
        match self.0.read_f32::<LittleEndian>() {
            Ok(v) => { Ok(v) }
            Err(e) => { Err(e) }
        }
    }

    /// Read a f32be
    #[inline]
    pub fn read_f32be(&mut self) -> Result<f32, std::io::Error> {
        match self.0.read_f32::<BigEndian>() {
            Ok(v) => { Ok(v) }
            Err(e) => { Err(e) }
        }
    }

    /// Read a f64le
    #[inline]
    pub fn read_f64le(&mut self) -> Result<f64, std::io::Error> {
        match self.0.read_f64::<LittleEndian>() {
            Ok(v) => { Ok(v) }
            Err(e) => { Err(e) }
        }
    }

    /// Read a f64be
    #[inline]
    pub fn read_f64be(&mut self) -> Result<f64, std::io::Error> {
        match self.0.read_f64::<BigEndian>() {
            Ok(v) => { Ok(v) }
            Err(e) => { Err(e) }
        }
    }

    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        self.0.read_exact(buf)
    }
}


impl Default for ByteStreamRead {
    #[inline]
    fn default() -> ByteStreamRead {
        ByteStreamRead::new()
    }
}

impl From<&'static [u8]> for ByteStreamRead {
    #[inline]
    fn from(slice: &'static [u8]) -> ByteStreamRead {
        ByteStreamRead::from_static(slice)
    }
}

impl From<&'static str> for ByteStreamRead {
    #[inline]
    fn from(slice: &'static str) -> ByteStreamRead {
        ByteStreamRead::from_static(slice.as_bytes())
    }
}

impl From<Vec<u8>> for ByteStreamRead {
    #[inline]
    fn from(vec: Vec<u8>) -> ByteStreamRead {
        ByteStreamRead::from_bytes(Bytes::from(vec))
    }
}

impl From<Box<[u8]>> for ByteStreamRead {
    #[inline]
    fn from(slice: Box<[u8]>) -> ByteStreamRead {
        ByteStreamRead::from_bytes(Bytes::from(slice))
    }
}

impl From<String> for ByteStreamRead {
    #[inline]
    fn from(s: String) -> ByteStreamRead {
        ByteStreamRead::from_bytes(Bytes::from(s))
    }
}

impl From<ByteStreamRead> for Vec<u8> {
    #[inline]
    fn from(bytes: ByteStreamRead) -> Vec<u8> {
        Vec::from(bytes.0.into_inner())
    }
}