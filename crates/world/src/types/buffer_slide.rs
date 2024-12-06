use bytemuck::Pod;
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::ops::{Deref, IndexMut};

pub struct SlideBuffer<'a, Container: IndexMut<usize, Output = u8> + ?Sized + len_trait::len::Len> {
    container: &'a mut Container,
    index: usize,
}

impl<'a, Container: IndexMut<usize, Output = u8> + len_trait::Len> SlideBuffer<'a, Container> {
    pub fn new(container: &'a mut Container) -> Self {
        Self {
            container,
            index: 0,
        }
    }

    pub fn write<T: Sized + bytemuck::Pod>(&mut self, val: T) -> &mut Self {
        let bytes = bytemuck::bytes_of(&val);
        for byte in bytes {
            self.push_byte(*byte);
        }
        self
    }

    pub fn push_byte(&mut self, val: u8) {
        self.index += 1;
        self.container[self.index - 1] = val;
    }

    pub fn size(&self) -> usize {
        self.container.len() - self.index
    }

    pub fn pos(&self) -> usize {
        self.index
    }
}
impl<
        'a,
        Container: IndexMut<usize, Output = u8> + ?Sized + len_trait::len::Len + Deref<Target = [u8]>,
    > Deref for SlideBuffer<'a, Container>
{
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.container.deref()
    }
}

pub struct BetterCursor<'a> {
    data: &'a [u8],
    index: usize,
}

#[allow(dead_code)]
impl<'a> BetterCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }

    pub fn read<T: Pod>(&mut self) -> Option<T> {
        if self.index + std::mem::size_of::<T>() <= self.data.len() {
            let value: T = *bytemuck::from_bytes(
                &self.data[self.index..self.index + std::mem::size_of::<T>()],
            );
            self.index += std::mem::size_of::<T>();
            Some(value)
        } else {
            None
        }
    }

    pub fn remaining(&self) -> &[u8] {
        &self.data[self.index..]
    }

    pub fn left(&self) -> usize {
        self.data.len() - self.index
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> Seek for BetterCursor<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(i) => {
                if i >= self.data.len() as u64 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Index was greater than buffer length",
                    ));
                }
                self.index = i as usize;
            }
            SeekFrom::End(i) => {
                let len = self.data.len() as i64;
                if len + i < 0 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Offset put buffer in the negatives",
                    ));
                } else if len + i > len - 1 {
                    return Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        "Offset put buffer over bounds",
                    ));
                }
                self.index = (len + i) as usize;
            }
            SeekFrom::Current(i) => {
                let len = self.data.len() as i64;
                if self.index as i64 + i > len - 1 || self.index as i64 + i < 0 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Offset put buffer out of bounds",
                    ));
                }
                self.index = (self.index as i64 + i) as usize;
            }
        }
        Ok(self.index as u64)
    }
}

impl<'a> AsRef<[u8]> for BetterCursor<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}
