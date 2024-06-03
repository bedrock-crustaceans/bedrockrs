use core::hash;
use std::borrow::Borrow;
use std::{cmp, slice};
use std::io::{BufRead, Cursor, Read};
use std::ops::Deref;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use bytes::buf::{IntoIter, Reader};
use bytes::{Buf, Bytes};
use varint_rs::VarintReader;

use crate::*;

/// A wrapper around [`bytes::Bytes`].
/// (A cheaply cloneable and sliceable chunk of contiguous memory).
#[repr(transparent)]
pub struct ByteStreamRead(Cursor<Bytes>);

impl ByteStreamRead {
    /// Creates a new empty `ByteStreamRead`.
    /// This will not allocate and the returned Bytes handle will be empty.
    #[inline]
    pub fn new() -> Self {
        Self(Cursor::new(Bytes::new()))
    }

    /// Creates a new empty `ByteStreamRead` from a given [`bytes::Bytes`] object.
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self(Cursor::new(bytes))
    }

    /// Creates a new `ByteStreamRead` from a static slice.
    #[inline]
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self(Cursor::new(Bytes::from_static(bytes)))
    }

    /// Returns the number of bytes contained in this `ByteStreamRead`.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.get_ref().len()
    }

    /// Advance the stream.
    #[inline]
    pub fn consume(&mut self, amt: usize) {
        self.0.advance(amt)
    }

    /// Returns the current position of this stream.
    #[inline]
    pub fn position(&self) -> u64 {
        self.0.position()
    }

    /// Returns the number of bytes between the current position and the end of the buffer.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.0.remaining()
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
            Ok(v) => Ok(u16le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u16be
    #[inline]
    pub fn read_u16be(&mut self) -> Result<u16be, std::io::Error> {
        match self.0.read_u16::<BigEndian>() {
            Ok(v) => Ok(u16be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i16le
    #[inline]
    pub fn read_i16le(&mut self) -> Result<i16le, std::io::Error> {
        match self.0.read_i16::<LittleEndian>() {
            Ok(v) => Ok(i16le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i16be
    #[inline]
    pub fn read_i16be(&mut self) -> Result<i16be, std::io::Error> {
        match self.0.read_i16::<BigEndian>() {
            Ok(v) => Ok(i16be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u32le
    #[inline]
    pub fn read_u32le(&mut self) -> Result<u32le, std::io::Error> {
        match self.0.read_u32::<LittleEndian>() {
            Ok(v) => Ok(u32le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u32be
    #[inline]
    pub fn read_u32be(&mut self) -> Result<u32be, std::io::Error> {
        match self.0.read_u32::<BigEndian>() {
            Ok(v) => Ok(u32be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i32le
    #[inline]
    pub fn read_i32le(&mut self) -> Result<i32le, std::io::Error> {
        match self.0.read_i32::<LittleEndian>() {
            Ok(v) => Ok(i32le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i32be
    #[inline]
    pub fn read_i32be(&mut self) -> Result<i32be, std::io::Error> {
        match self.0.read_i32::<BigEndian>() {
            Ok(v) => Ok(i32be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u64le
    #[inline]
    pub fn read_u64le(&mut self) -> Result<u64le, std::io::Error> {
        match self.0.read_u64::<LittleEndian>() {
            Ok(v) => Ok(u64le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u64be
    #[inline]
    pub fn read_u64be(&mut self) -> Result<u64be, std::io::Error> {
        match self.0.read_u64::<BigEndian>() {
            Ok(v) => Ok(u64be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i64le
    #[inline]
    pub fn read_i64le(&mut self) -> Result<i64le, std::io::Error> {
        match self.0.read_i64::<LittleEndian>() {
            Ok(v) => Ok(i64le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i64be
    #[inline]
    pub fn read_i64be(&mut self) -> Result<i64be, std::io::Error> {
        match self.0.read_i64::<BigEndian>() {
            Ok(v) => Ok(i64be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u128le
    #[inline]
    pub fn read_u128le(&mut self) -> Result<u128le, std::io::Error> {
        match self.0.read_u128::<LittleEndian>() {
            Ok(v) => Ok(u128le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an u128be
    #[inline]
    pub fn read_u128be(&mut self) -> Result<u128be, std::io::Error> {
        match self.0.read_u128::<BigEndian>() {
            Ok(v) => Ok(u128be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i128le
    #[inline]
    pub fn read_i128le(&mut self) -> Result<i128le, std::io::Error> {
        match self.0.read_i128::<LittleEndian>() {
            Ok(v) => Ok(i128le(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an i128be
    #[inline]
    pub fn read_i128be(&mut self) -> Result<i128be, std::io::Error> {
        match self.0.read_i128::<BigEndian>() {
            Ok(v) => Ok(i128be(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an uvar32
    #[inline]
    pub fn read_uvar32(&mut self) -> Result<uvar32, std::io::Error> {
        match self.0.read_u32_varint() {
            Ok(v) => Ok(uvar32(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an ivar32
    #[inline]
    pub fn read_ivar32(&mut self) -> Result<ivar32, std::io::Error> {
        match self.0.read_i32_varint() {
            Ok(v) => Ok(ivar32(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an uvar64
    #[inline]
    pub fn read_uvar64(&mut self) -> Result<uvar64, std::io::Error> {
        match self.0.read_u64_varint() {
            Ok(v) => Ok(uvar64(v)),
            Err(e) => Err(e),
        }
    }

    /// Read an ivar64
    #[inline]
    pub fn read_ivar64(&mut self) -> Result<ivar64, std::io::Error> {
        match self.0.read_i64_varint() {
            Ok(v) => Ok(ivar64(v)),
            Err(e) => Err(e),
        }
    }

    /// Read a f32le
    #[inline]
    pub fn read_f32le(&mut self) -> Result<f32, std::io::Error> {
        match self.0.read_f32::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Read a f32be
    #[inline]
    pub fn read_f32be(&mut self) -> Result<f32, std::io::Error> {
        match self.0.read_f32::<BigEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Read a f64le
    #[inline]
    pub fn read_f64le(&mut self) -> Result<f64, std::io::Error> {
        match self.0.read_f64::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Read a f64be
    #[inline]
    pub fn read_f64be(&mut self) -> Result<f64, std::io::Error> {
        match self.0.read_f64::<BigEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        self.0.read_exact(buf)
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.get_ref().as_ptr(), self.len()) }
    }
}

impl Read for ByteStreamRead {
    #[inline]
    fn read(&mut self, dst: &mut [u8]) -> std::io::Result<usize> {
        std::io::Read::read(&mut self.0, dst)
    }
}

impl Buf for ByteStreamRead {
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


impl Deref for ByteStreamRead {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<[u8]> for ByteStreamRead {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl hash::Hash for ByteStreamRead {
    #[inline]
    fn hash<H>(&self, state: &mut H)
        where
            H: hash::Hasher,
    {
        self.as_slice().hash(state);
    }
}

impl Borrow<[u8]> for ByteStreamRead {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl IntoIterator for ByteStreamRead {
    type Item = u8;
    type IntoIter = IntoIter<ByteStreamRead>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a ByteStreamRead {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl FromIterator<u8> for ByteStreamRead {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        Vec::from_iter(into_iter).into()
    }
}

// impl Eq

impl PartialEq for ByteStreamRead {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialOrd for ByteStreamRead {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for ByteStreamRead {
    #[inline]
    fn cmp(&self, other: &ByteStreamRead) -> cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Eq for ByteStreamRead {}

impl PartialEq<[u8]> for ByteStreamRead {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl PartialOrd<[u8]> for ByteStreamRead {
    #[inline]
    fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }
}

impl PartialEq<ByteStreamRead> for [u8] {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for [u8] {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<str> for ByteStreamRead {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl PartialOrd<str> for ByteStreamRead {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_bytes())
    }
}

impl PartialEq<ByteStreamRead> for str {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for str {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl PartialEq<Vec<u8>> for ByteStreamRead {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        *self == other[..]
    }
}

impl PartialOrd<Vec<u8>> for ByteStreamRead {
    #[inline]
    fn partial_cmp(&self, other: &Vec<u8>) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(&other[..])
    }
}

impl PartialEq<ByteStreamRead> for Vec<u8> {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for Vec<u8> {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<String> for ByteStreamRead {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        *self == other[..]
    }
}

impl PartialOrd<String> for ByteStreamRead {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_bytes())
    }
}

impl PartialEq<ByteStreamRead> for String {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for String {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl PartialEq<ByteStreamRead> for &[u8] {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for &[u8] {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<ByteStreamRead> for &str {
    #[inline]
    fn eq(&self, other: &ByteStreamRead) -> bool {
        *other == *self
    }
}

impl PartialOrd<ByteStreamRead> for &str {
    #[inline]
    fn partial_cmp(&self, other: &ByteStreamRead) -> Option<cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl<'a, T: ?Sized> PartialEq<&'a T> for ByteStreamRead
    where
        ByteStreamRead: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
}

impl<'a, T: ?Sized> PartialOrd<&'a T> for ByteStreamRead
    where
        ByteStreamRead: PartialOrd<T>,
{
    #[inline]
    fn partial_cmp(&self, other: &&'a T) -> Option<cmp::Ordering> {
        self.partial_cmp(&**other)
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
