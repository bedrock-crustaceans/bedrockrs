use std::io::{self, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct LE<T> {
    num: T,
}

impl<T> LE<T> {
    #[inline]
    fn new(num: T) -> Self {
        Self { num }
    }

    #[inline]
    fn into_inner(self) -> T {
        self.num
    }
}

macro_rules! impl_le {
    ($type:ty, $read_fn_name:ident, $write_fn_name:ident) => {
        impl LE<$type> {
            #[inline]
            fn $read_fn_name<R: Read>(reader: &mut R) -> io::Result<Self> {
                let num = reader.$read_fn_name::<LittleEndian>()?;
                Ok(LE::new(num))
            }

            #[inline]
            fn $write_fn_name<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                writer.$write_fn_name::<LittleEndian>(self.num)?;
                Ok(())
            }
        }
    };
}

impl LE<u8> {
    fn read_u8<R: Read>(reader: &mut R) -> io::Result<Self> {
        let num = reader.read_u8()?;
        Ok(LE::new(num))
    }

    fn write_u8<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(self.num)?;
        Ok(())
    }
}

impl LE<i8> {
    fn read_i8<R: Read>(reader: &mut R) -> io::Result<Self> {
        let num = reader.read_i8()?;
        Ok(LE::new(num))
    }

    fn write_i8<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i8(self.num)?;
        Ok(())
    }
}

impl_le!(u16, read_u16, write_u16);
impl_le!(i16, read_i16, write_i16);

impl_le!(u32, read_u32, write_u32);
impl_le!(i32, read_i32, write_i32);

impl_le!(u64, read_u64, write_u64);
impl_le!(i64, read_i64, write_i64);

impl_le!(u128, read_u128, write_u128);
impl_le!(i128, read_i128, write_i128);
