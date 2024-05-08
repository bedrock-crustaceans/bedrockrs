use std::io::{Cursor, Read, Write};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use varint_rs::{VarintReader, VarintWriter};

use crate::error::{DeserilizationError, SerilizationError};
use crate::proto::de::MCProtoDeserialize;
use crate::proto::ser::MCProtoSerialize;

// i8
impl MCProtoSerialize for i8 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i8(*self) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for i8 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i8() {
            Ok(v) => Ok(v),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// u8
impl MCProtoSerialize for u8 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u8(*self) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for u8 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u8() {
            Ok(v) => Ok(v),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// i16
impl MCProtoSerialize for bedrock_core::types::i16le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i16::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i16le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i16::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::i16be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i16::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i16be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i16::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// u16
impl MCProtoSerialize for bedrock_core::types::u16le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u16::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u16le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u16::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::u16be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u16::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u16be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u16::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// i32

impl MCProtoSerialize for bedrock_core::types::i32le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i32::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i32le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i32::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::i32be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i32::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i32be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i32::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// u32

impl MCProtoSerialize for bedrock_core::types::u32le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u32::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u32le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u32::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::u32be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u32::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u32be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u32::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// i64

impl MCProtoSerialize for bedrock_core::types::i64le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i64::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i64le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i64::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::i64be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i64::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i64be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i64::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// u64

impl MCProtoSerialize for bedrock_core::types::u64le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u64::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u64le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u64::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// i128

impl MCProtoSerialize for bedrock_core::types::i128le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i128::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i128le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i128::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::i128be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i128::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::i128be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i128::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// u128

impl MCProtoSerialize for bedrock_core::types::u128le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u128::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u128le {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u128::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

impl MCProtoSerialize for bedrock_core::types::u128be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u128::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::u128be {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u128::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// f32

impl MCProtoSerialize for f32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_f32::<LittleEndian>(*self) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for f32 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_f32::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// f64

impl MCProtoSerialize for f64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_f64::<LittleEndian>(*self) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for f64 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_f64::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// uvarint 32

impl MCProtoSerialize for bedrock_core::types::uvar32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u32_varint(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::uvar32 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_u32_varint() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(DeserilizationError::ReadIOError),
        }
    }
}

// ivarint 32

impl MCProtoSerialize for bedrock_core::types::ivar32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i32_varint(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::ivar32 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_i32_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadIOError),
        }));
    }
}

// uvarint 64

impl MCProtoSerialize for bedrock_core::types::uvar64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u64_varint(self.0) {
            Ok(_) => {}
            Err(_) => return Err(SerilizationError::WriteIOError),
        };

        Ok(())
    }
}

impl MCProtoDeserialize for bedrock_core::types::uvar64 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_u64_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadIOError),
        }));
    }
}

// ivarint 64

impl MCProtoSerialize for bedrock_core::types::ivar64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_i64_varint(self.0) {
            Ok(_) => Ok(()),
            Err(_) => Err(SerilizationError::WriteIOError),
        }
    }
}

impl MCProtoDeserialize for bedrock_core::types::ivar64 {
    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match data.read_i64_varint() {
            Ok(v) => Ok(Self(v)),
            Err(_) => return Err(DeserilizationError::ReadIOError),
        }
    }
}

// bool

impl MCProtoSerialize for bool {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match self {
            true => match buf.write_u8(1) {
                Ok(_) => Ok(()),
                Err(_) => Err(SerilizationError::WriteIOError),
            },
            false => match buf.write_u8(0) {
                Ok(_) => Ok(()),
                Err(_) => Err(SerilizationError::WriteIOError),
            },
        }
    }
}

impl MCProtoDeserialize for bool {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        // a bool is represented as a byte
        return match cursor.read_u8() {
            Ok(v) => {
                match v {
                    // 0 is counted as false
                    0 => Ok(false),
                    // Anything about 0 is counted as true
                    _ => Ok(true),
                }
            }
            Err(_) => Err(DeserilizationError::ReadIOError),
        };
    }
}

// string

impl MCProtoSerialize for String {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u64_varint(self.len() as u64) {
            Ok(_) => {}
            Err(_) => return Err(SerilizationError::WriteIOError),
        };

        match buf.write_all(self.as_bytes()) {
            Ok(_) => {}
            Err(_) => return Err(SerilizationError::WriteIOError),
        };

        Ok(())
    }
}

impl MCProtoDeserialize for String {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        let len = match cursor.read_u64_varint() {
            Ok(v) => v,
            Err(_) => {
                return Err(DeserilizationError::ReadIOError);
            }
        };

        let mut string_buf = vec![0u8; len as usize];

        match cursor.read_exact(&mut *string_buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(DeserilizationError::NotEnoughRemainingError);
            }
        }

        match String::from_utf8(string_buf) {
            Ok(str) => Ok(str),
            Err(_) => Err(DeserilizationError::ReadUtf8StringError),
        }
    }
}

// vec

impl<T: MCProtoSerialize> MCProtoSerialize for Vec<T> {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match buf.write_u64_varint(self.len() as u64) {
            Ok(_) => {}
            Err(_) => return Err(SerilizationError::WriteIOError),
        };

        for item in self {
            match item.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        Ok(())
    }
}

// option

impl<T: MCProtoSerialize> MCProtoSerialize for Option<T> {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match self {
            None => match false.proto_serialize(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Some(v) => {
                match true.proto_serialize(buf) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e),
                };

                match v.proto_serialize(buf) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

impl<T: MCProtoDeserialize> MCProtoDeserialize for Option<T> {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        match bool::proto_deserialize(cursor) {
            Ok(v) => match v {
                false => Ok(Option::None),
                true => match T::proto_deserialize(cursor) {
                    Ok(v) => Ok(Option::Some(v)),
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(e),
        }
    }
}
