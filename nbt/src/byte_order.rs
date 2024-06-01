use bedrock_core::stream::read::ByteStreamRead;

use crate::error::NbtError;

pub trait NbtByteOrder {
    fn write_u8(buf: &mut Vec<u8>, byte: u8) -> Result<(), NbtError>;
    fn write_i16(buf: &mut Vec<u8>, int16: i16) -> Result<(), NbtError>;
    fn write_i32(buf: &mut Vec<u8>, int32: i32) -> Result<(), NbtError>;
    fn write_i64(buf: &mut Vec<u8>, int64: i64) -> Result<(), NbtError>;
    fn write_f32(buf: &mut Vec<u8>, int64: f32) -> Result<(), NbtError>;
    fn write_f64(buf: &mut Vec<u8>, int64: f64) -> Result<(), NbtError>;
    fn write_string(buf: &mut Vec<u8>, int64: String) -> Result<(), NbtError>;

    fn read_u8(buf: &mut ByteStreamRead) -> Result<u8, NbtError>;
    fn read_i16(buf: &mut ByteStreamRead) -> Result<i16, NbtError>;
    fn read_i32(buf: &mut ByteStreamRead) -> Result<i32, NbtError>;
    fn read_i64(buf: &mut ByteStreamRead) -> Result<i64, NbtError>;
    fn read_f32(buf: &mut ByteStreamRead) -> Result<f32, NbtError>;
    fn read_f64(buf: &mut ByteStreamRead) -> Result<f64, NbtError>;
    fn read_string(buf: &mut ByteStreamRead) -> Result<String, NbtError>;
}
