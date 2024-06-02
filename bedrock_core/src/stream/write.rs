use core::{fmt, hash};
use std::borrow::{Borrow, BorrowMut};
use std::{cmp, slice};
use std::hash::Hash;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use bytes::buf::{IntoIter, Writer};
use bytes::{Buf, BufMut, BytesMut};
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

    /// Write a f32le
    #[inline]
    pub fn write_f32le(&mut self, n: f32) -> Result<(), std::io::Error> {
        match self.0.write_f32::<LittleEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write a f32be
    #[inline]
    pub fn write_f32be(&mut self, n: f32) -> Result<(), std::io::Error> {
        match self.0.write_f32::<BigEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Write a f64le
    #[inline]
    pub fn write_f64le(&mut self, n: f64) -> Result<(), std::io::Error> {
        match self.0.write_f64::<LittleEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }


    /// Write a f64be
    #[inline]
    pub fn write_f64be(&mut self, n: f64) -> Result<(), std::io::Error> {
        match self.0.write_f64::<LittleEndian>(n) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.get_ref().as_ptr(), self.len()) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.0.get_mut().as_mut_ptr(), self.len()) }
    }
}

impl Write for ByteStreamWrite {
    #[inline]
    fn write(&mut self, src: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(&mut self.0, src)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}


impl Buf for ByteStreamWrite {
    #[inline]
    fn remaining(&self) -> usize {
        self.0.get_ref().remaining()
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.0.get_ref().chunk()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.0.get_mut().advance(cnt)
    }
}

impl AsRef<[u8]> for ByteStreamWrite {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for ByteStreamWrite {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsMut<[u8]> for ByteStreamWrite {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl DerefMut for ByteStreamWrite {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl<'a> From<&'a [u8]> for ByteStreamWrite {
    #[inline]
    fn from(src: &'a [u8]) -> ByteStreamWrite {
        ByteStreamWrite::from(src)
    }
}

impl<'a> From<&'a str> for ByteStreamWrite {
    #[inline]
    fn from(src: &'a str) -> ByteStreamWrite {
        ByteStreamWrite::from(src.as_bytes())
    }
}

impl From<ByteStreamWrite> for ByteStreamRead {
    #[inline]
    fn from(src: ByteStreamWrite) -> ByteStreamRead {
        src.freeze()
    }
}

impl PartialEq for ByteStreamWrite {
    #[inline]
    fn eq(&self, other: &ByteStreamWrite) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialOrd for ByteStreamWrite {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamWrite) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for ByteStreamWrite {
    #[inline]
    fn cmp(&self, other: &ByteStreamWrite) -> cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Eq for ByteStreamWrite {}

impl Default for ByteStreamWrite {
    #[inline]
    fn default() -> ByteStreamWrite {
        ByteStreamWrite::new()
    }
}

impl hash::Hash for ByteStreamWrite {
    #[inline]
    fn hash<H>(&self, state: &mut H)
        where
            H: hash::Hasher,
    {
        let s: &[u8] = self.as_ref();
        s.hash(state);
    }
}

impl Borrow<[u8]> for ByteStreamWrite {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl BorrowMut<[u8]> for ByteStreamWrite {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl fmt::Write for ByteStreamWrite {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.get_mut().write_str(s)
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(self, args)
    }
}

impl Clone for ByteStreamWrite {
    #[inline]
    fn clone(&self) -> ByteStreamWrite {
        ByteStreamWrite::from(&self[..])
    }
}

impl IntoIterator for ByteStreamWrite {
    type Item = u8;
    type IntoIter = IntoIter<ByteStreamWrite>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a ByteStreamWrite {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl Extend<u8> for ByteStreamWrite {
    #[inline]
    fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = u8>,
    {
        self.extend(iter)
    }
}

impl<'a> Extend<&'a u8> for ByteStreamWrite {
    #[inline]
    fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = &'a u8>,
    {
        self.extend(iter.into_iter().copied())
    }
}

impl Extend<ByteStreamWrite> for ByteStreamWrite {
    #[inline]
    fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = ByteStreamWrite>,
    {
        for bytes in iter {
            self.0.get_mut().extend_from_slice(&bytes)
        }
    }
}

impl FromIterator<u8> for ByteStreamWrite {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        ByteStreamWrite::from(&*Vec::from_iter(into_iter))
    }
}

impl<'a> FromIterator<&'a u8> for ByteStreamWrite {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self {
        ByteStreamWrite::from_iter(into_iter.into_iter().copied())
    }
}

