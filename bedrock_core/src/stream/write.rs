use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use bytes::buf::Writer;
use bytes::{BufMut, BytesMut};
use varint_rs::{VarintWriter};

use crate::stream::read::ByteStreamRead;
use crate::*;

/// A wrapper around [`bytes::BytesMut`].
/// (A unique reference to a contiguous slice of memory).
#[repr(transparent)]
pub struct ByteStreamWrite(Writer<BytesMut>);

impl ByteStreamWrite {
    /// Creates a new `ByteStreamWrite` with default capacity.
    /// Resulting object has length 0 and unspecified capacity. This function does not allocate.
    #[inline]
    pub fn new() -> Self {
        Self(BytesMut::new().writer())
    }

    /// Creates a new empty `ByteStreamWrite` from a given [`bytes::BytesMut`] object.
    #[inline]
    pub fn from_bytes_mut(bytes: BytesMut) -> Self {
        Self(bytes.writer())
    }

    /// Returns the number of bytes contained in this `ByteStreamWrite`.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.get_ref().len()
    }

    /// Returns true if the `ByteStreamWrite` has a length of 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.get_ref().is_empty()
    }

    /// Returns the number of bytes the `ByteStreamWrite` can hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.get_ref().capacity()
    }

    /// Converts self into an immutable `ByteStreamWrite`.
    #[inline]
    pub fn freeze(self) -> ByteStreamRead {
        ByteStreamRead::from_bytes(self.0.into_inner().freeze())
    }

    /// Creates a new `ByteStreamWrite`, which is initialized with zero.
    #[inline]
    pub fn zeroed(len: usize) -> Self {
        Self(BytesMut::zeroed(len).writer())
    }

    /// Clears the buffer, removing all data. Existing capacity is preserved.
    #[inline]
    pub fn clear(&mut self) {
        self.0.get_mut().clear();
    }

    /// Write an u8
    #[inline]
    pub fn write_u8(&mut self, n: u8) -> Result<(), std::io::Error> {
        self.0.write_u8(n)
    }

    /// Write an i8
    #[inline]
    pub fn write_i8(&mut self, n: i8) -> Result<(), std::io::Error> {
        self.0.write_i8(n)
    }

    /// Write an u16le
    #[inline]
    pub fn write_u16le(&mut self, n: u16le) -> Result<(), std::io::Error> {
        match self.0.write_u16::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u16be
    #[inline]
    pub fn write_u16be(&mut self, n: u16be) -> Result<(), std::io::Error> {
        match self.0.write_u16::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i16le
    #[inline]
    pub fn write_i16le(&mut self, n: i16le) -> Result<(), std::io::Error> {
        match self.0.write_i16::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i16be
    #[inline]
    pub fn write_i16be(&mut self, n: i16be) -> Result<(), std::io::Error> {
        match self.0.write_i16::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u32le
    #[inline]
    pub fn write_u32le(&mut self, n: u32le) -> Result<(), std::io::Error> {
        match self.0.write_u32::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u32be
    #[inline]
    pub fn write_u32be(&mut self, n: u32be) -> Result<(), std::io::Error> {
        match self.0.write_u32::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i32le
    #[inline]
    pub fn write_i32le(&mut self, n: i32le) -> Result<(), std::io::Error> {
        match self.0.write_i32::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i32be
    #[inline]
    pub fn write_i32be(&mut self, n: i32be) -> Result<(), std::io::Error> {
        match self.0.write_i32::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u64le
    #[inline]
    pub fn write_u64le(&mut self, n: u64le) -> Result<(), std::io::Error> {
        match self.0.write_u64::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u64be
    #[inline]
    pub fn write_u64be(&mut self, n: u64be) -> Result<(), std::io::Error> {
        match self.0.write_u64::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i64le
    #[inline]
    pub fn write_i64le(&mut self, n: i64le) -> Result<(), std::io::Error> {
        match self.0.write_i64::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i64be
    #[inline]
    pub fn write_i64be(&mut self, n: i64be) -> Result<(), std::io::Error> {
        match self.0.write_i64::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u128le
    #[inline]
    pub fn write_u128le(&mut self, n: u128le) -> Result<(), std::io::Error> {
        match self.0.write_u128::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an u128be
    #[inline]
    pub fn write_u128be(&mut self, n: u128be) -> Result<(), std::io::Error> {
        match self.0.write_u128::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i128le
    #[inline]
    pub fn write_i128le(&mut self, n: i128le) -> Result<(), std::io::Error> {
        match self.0.write_i128::<LittleEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an i128be
    #[inline]
    pub fn write_i128be(&mut self, n: i128be) -> Result<(), std::io::Error> {
        match self.0.write_i128::<BigEndian>(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an uvar32
    #[inline]
    pub fn write_uvar32(&mut self, n: uvar32) -> Result<(), std::io::Error> {
        match self.0.write_u32_varint(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an ivar32
    #[inline]
    pub fn write_ivar32(&mut self, n: ivar32) -> Result<(), std::io::Error> {
        match self.0.write_i32_varint(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an uvar64
    #[inline]
    pub fn write_uvar64(&mut self, n: uvar64) -> Result<(), std::io::Error> {
        match self.0.write_u64_varint(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write an ivar64
    #[inline]
    pub fn write_ivar64(&mut self, n: ivar64) -> Result<(), std::io::Error> {
        match self.0.write_i64_varint(n.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write a f32
    #[inline]
    pub fn write_f32(&mut self, n: f32) -> Result<(), std::io::Error> {
        match self.0.write_f32::<LittleEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write a f64
    #[inline]
    pub fn write_f64(&mut self, n: f64) -> Result<(), std::io::Error> {
        match self.0.write_f64::<BigEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
