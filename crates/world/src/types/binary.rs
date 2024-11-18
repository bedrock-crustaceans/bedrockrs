use byteorder::ByteOrder;
use len_trait::Len;
use mojang_leveldb::LevelDBManagedBytes;
use thiserror::Error;

#[derive(Debug)]
pub struct BinaryBuffer {
    buffer: Vec<u8>,
    index: usize,
}

pub trait BinaryInterface: Sized {
    fn write<Writer: ByteOrder>(
        buff: &mut BinaryBuffer,
        value: Self,
    ) -> Result<(), BinaryInterfaceError>;
    fn read<Reader: ByteOrder>(buff: &mut BinaryBuffer) -> Option<Self>;
}

#[derive(Error, Debug)]
pub enum BinaryInterfaceError {
    #[error("There was not enough space in the binary buffer to perform the requested action: Needed {0} Actual {1}")]
    OutOfSpace(usize, usize),
}

macro_rules! generate_integral_type_impl {
    ($id:ident) => {
        impl BinaryInterface for $id {
            concat_idents::concat_idents!(full_write = write_, $id{
            fn write<Writer: ByteOrder>(buff: &mut BinaryBuffer, value: Self) -> Result<(), BinaryInterfaceError> {
                let buffer = buff.poll_buff_ext(std::mem::size_of::<$id>());
                Writer::full_write(buffer, value);
                buff.rebase(std::mem::size_of::<$id>() as isize);
                Ok(())
            }});
            concat_idents::concat_idents!(full_read = read_, $id {
                fn read<Reader: ByteOrder>(buff: &mut BinaryBuffer) -> Option<Self> {
                    let result = Reader::full_read(buff.poll_buffer()?);
                    buff.rebase(std::mem::size_of::<$id>() as isize);
                    Some(result)
                }
            });
        }
    };
}
impl BinaryInterface for u8 {
    fn write<Writer: ByteOrder>(
        buff: &mut BinaryBuffer,
        value: Self,
    ) -> Result<(), BinaryInterfaceError> {
        let buffer = buff.poll_buff_ext(1);
        Writer::write_uint(buffer, value as u64, 1);
        buff.rebase(1);
        Ok(())
    }
    fn read<Reader: ByteOrder>(buff: &mut BinaryBuffer) -> Option<Self> {
        let result = Reader::read_uint(buff.poll_buffer()?, 1) as u8;
        buff.rebase(1);
        Some(result)
    }
}
impl BinaryInterface for i8 {
    fn write<Writer: ByteOrder>(
        buff: &mut BinaryBuffer,
        value: Self,
    ) -> Result<(), BinaryInterfaceError> {
        let buffer = buff.poll_buff_ext(1);
        Writer::write_int(buffer, value as i64, 1);
        buff.rebase(1);
        Ok(())
    }
    fn read<Reader: ByteOrder>(buff: &mut BinaryBuffer) -> Option<Self> {
        let result = Some(Reader::read_uint(buff.poll_buffer()?, 1) as i8);
        buff.rebase(1);
        result
    }
}
impl BinaryInterface for &[u8] {
    fn write<Writer: ByteOrder>(
        buff: &mut BinaryBuffer,
        value: Self,
    ) -> Result<(), BinaryInterfaceError> {
        let buffer = buff.poll_buff_ext(value.len());
        for idx in 0..value.len() {
            buffer[idx] = value[idx]
        }
        buff.rebase(value.len() as isize);
        Ok(())
    }
    fn read<Reader: ByteOrder>(_: &mut BinaryBuffer) -> Option<Self> {
        unimplemented!()
    }
}
generate_integral_type_impl!(u16);
generate_integral_type_impl!(i16);
generate_integral_type_impl!(u32);
generate_integral_type_impl!(i32);
generate_integral_type_impl!(u64);
generate_integral_type_impl!(i64);
generate_integral_type_impl!(f32);
generate_integral_type_impl!(f64);

impl BinaryBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            index: 0,
        }
    }

    pub fn write<Writer: ByteOrder, T: BinaryInterface>(
        &mut self,
        val: T,
    ) -> Result<&mut Self, BinaryInterfaceError> {
        T::write::<Writer>(self, val)?;
        Ok(self)
    }

    pub fn read<Reader: ByteOrder, T: BinaryInterface>(&mut self) -> Option<T> {
        T::read::<Reader>(self)
    }

    pub fn extend(&mut self, min_req: usize) -> &mut Self {
        let size = self.buffer.capacity();
        if size < min_req {
            self.buffer.resize(min_req, 0);
        } else {
            self.buffer.resize(size * 2, 0);
        }
        self
    }

    pub fn poll_buffer(&mut self) -> Option<&mut [u8]> {
        let len = self.buffer.len();
        if len <= self.index {
            None
        } else {
            Some(&mut self.buffer[self.index..len])
        }
    }

    fn can_poll(&self, offset: usize) -> bool {
        !(self.buffer.len() <= self.index + offset - 1)
    }

    pub fn poll_buff_ext(&mut self, min_req: usize) -> &mut [u8] {
        if self.can_poll(min_req) {
            self.poll_buffer().unwrap()
        } else {
            self.extend(min_req);
            self.poll_buff_ext(min_req)
        }
    }

    pub fn rebase(&mut self, distance: isize) {
        let new = self.index as isize + distance;
        self.index = new as usize;
    }

    pub fn allocate_extra(&mut self, count: usize) {
        self.buffer.reserve(count);
    }

    pub fn get_remaining(&self) -> usize {
        self.buffer.len() - self.index
    }
    pub fn get_remaining_mut(&mut self) -> usize {
        self.buffer.len() - self.index
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn get_pointer(&self) -> usize {
        self.index
    }

    pub fn overall_size(&self) -> usize {
        self.buffer.len()
    }
}

impl From<BinaryBuffer> for Vec<u8> {
    fn from(value: BinaryBuffer) -> Self {
        value.buffer
    }
}

impl From<Vec<u8>> for BinaryBuffer {
    fn from(value: Vec<u8>) -> Self {
        Self {
            buffer: value,
            index: 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::types::binary::BinaryBuffer;
    use anyhow::anyhow;
    use byteorder::{BigEndian, LittleEndian};

    #[test]
    fn binary_buffer_test() -> Result<(), anyhow::Error> {
        let mut buff: Vec<u8> = Vec::with_capacity(256);
        unsafe {
            buff.set_len(256);
        }
        let mut buff: BinaryBuffer = buff.into();
        buff.write::<LittleEndian, i32>(2567)?.reset();
        assert_eq!(
            2567,
            buff.read::<LittleEndian, i32>()
                .ok_or(anyhow!("Read was empty"))?
        );
        buff.write::<BigEndian, i32>(2567)?.reset();
        assert_eq!(
            2567_i32.to_be(),
            buff.read::<BigEndian, i32>()
                .ok_or(anyhow!("Read was empty"))?
        );
        buff.reset();

        buff.write::<LittleEndian, i32>(01234)?
            .write::<LittleEndian, i32>(56789)?
            .reset();
        assert_eq!(
            01234,
            buff.read::<LittleEndian, i32>()
                .ok_or(anyhow!("Read Empty"))?
        );
        assert_eq!(
            56789,
            buff.read::<LittleEndian, i32>()
                .ok_or(anyhow!("Read Empty"))?
        );

        Ok(())
    }

    #[test]
    fn auto_allocation_test() -> Result<(), anyhow::Error> {
        let mut buff: BinaryBuffer = Vec::new().into();
        buff.write::<LittleEndian, i32>(2567)?.reset();
        assert_eq!(
            2567,
            buff.read::<LittleEndian, i32>()
                .ok_or(anyhow!("Read was empty"))?
        );
        let mut buff: BinaryBuffer = Vec::new().into();
        buff.write::<LittleEndian, &[u8]>(&[1, 2, 3, 4, 5])?.reset();
        for x in 1..=5 {
            assert_eq!(x, buff.read::<LittleEndian, u8>().unwrap());
        }

        Ok(())
    }
}
