use std::io;
use std::io::{Read, Write};

use varint_rs::{VarintReader, VarintWriter};

pub struct VAR<T> {
    num: T,
}

impl<T> VAR<T> {
    #[inline]
    fn new(num: T) -> Self {
        Self { num }
    }

    #[inline]
    fn into_inner(self) -> T {
        self.num
    }
}

macro_rules! impl_var {
    ($type:ty, $read_fn_name:ident, $Iread_fn_name:ident, $write_fn_name:ident, $Iwrite_fn_name:ident) => {
        impl VAR<$type> {
            #[inline]
            fn $read_fn_name<R: Read>(reader: &mut R) -> io::Result<Self> {
                let num = reader.$Iread_fn_name()?;
                Ok(VAR::new(num))
            }

            #[inline]
            fn $write_fn_name<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                writer.$Iwrite_fn_name(self.num)?;
                Ok(())
            }
        }
    };
}

impl_var!(u16, read_u16, read_u16_varint, write_u16, write_u16_varint);
impl_var!(i16, read_i16, read_i16_varint, write_i16, write_i16_varint);

impl_var!(u32, read_u32, read_u32_varint, write_u32, write_u32_varint);
impl_var!(i32, read_i32, read_i32_varint, write_i32, write_i32_varint);

impl_var!(u64, read_u64, read_u64_varint, write_u64, write_u64_varint);
impl_var!(i64, read_i64, read_i64_varint, write_i64, write_i64_varint);

impl_var!(
    u128,
    read_u128,
    read_u128_varint,
    write_u128,
    write_u128_varint
);
impl_var!(
    i128,
    read_i128,
    read_i128_varint,
    write_i128,
    write_i128_varint
);
