use std::f32;
use std::io::{Cursor, Read, Write};

use bedrock_core::*;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use varint_rs::{VarintReader, VarintWriter};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

// i8
impl ProtoCodec for i8 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i8() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u8
impl ProtoCodec for u8 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u8() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i16
impl ProtoCodec for i16le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i16::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i16::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i16be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i16::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i16::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u16
impl ProtoCodec for u16le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u16::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u16::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u16be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u16::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u16::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i32

impl ProtoCodec for i32le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i32::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i32::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i32be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i32::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i32::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u32

impl ProtoCodec for u32le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u32::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u32::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u32be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u32::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u32::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i64

impl ProtoCodec for i64le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i64::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i64::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i64be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i64::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i64::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u64

impl ProtoCodec for u64le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u64::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u64::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i128

impl ProtoCodec for i128le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i128::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i128::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i128be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i128::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i128::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u128

impl ProtoCodec for u128le {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u128::<LittleEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u128::<LittleEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u128be {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u128::<BigEndian>(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u128::<BigEndian>() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// f32

impl ProtoCodec for f32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_f32::<LittleEndian>(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_f32::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// f64

impl ProtoCodec for f64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_f64::<LittleEndian>(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_f64::<LittleEndian>() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// uvarint 32

impl ProtoCodec for uvar32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u32_varint(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u32_varint() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// ivarint 32

impl ProtoCodec for ivar32 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i32_varint(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i32_varint() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// uvarint 64

impl ProtoCodec for uvar64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u64_varint(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_u64_varint() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// ivarint 64

impl ProtoCodec for ivar64 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_i64_varint(self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match data.read_i64_varint() {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// bool

impl ProtoCodec for bool {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self {
            true => match buf.write_u8(1) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(e)),
            },
            false => match buf.write_u8(0) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(e)),
            },
        }
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        // a bool is represented as a byte
        return match cursor.read_u8() {
            Ok(v) => {
                match v {
                    // 0 is counted as false
                    0 => Ok(false),
                    // Anything above 0 is true
                    _ => Ok(true),
                }
            }
            Err(e) => Err(ProtoCodecError::IOError(e)),
        };
    }
}

// String

impl ProtoCodec for String {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u32_varint(self.len() as u32) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        match buf.write_all(self.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        let len = match cursor.read_u32_varint() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        };

        let mut string_buf = vec![0u8; len as usize];

        match cursor.read_exact(&mut *string_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        }

        match String::from_utf8(string_buf) {
            Ok(str) => Ok(str),
            Err(e) => Err(ProtoCodecError::UTF8Error(e)),
        }
    }
}

// vec

impl<T: ProtoCodec> ProtoCodec for Vec<T> {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match buf.write_u32_varint(self.len() as u32) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        for item in self {
            match item.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        let len = match cursor.read_u32_varint() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        };

        let mut array = vec![];

        for _ in 0..len {
            array.push(match T::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            })
        }

        Ok(array)
    }
}

// option

impl<T: ProtoCodec> ProtoCodec for Option<T> {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
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
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };

                match v.proto_serialize(buf) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
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

impl ProtoCodec for Vec2 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self.x.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.z.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        Ok(Self {
            x: match i32le::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match i32le::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec2f {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self.x.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.z.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        Ok(Self {
            x: match f32::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match f32::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3 {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self.x.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.y.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.z.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        Ok(Self {
            x: match i32le::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            y: match i32le::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match i32le::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3f {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self.x.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.y.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match self.z.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        Ok(Self {
            x: match f32::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            y: match f32::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match f32::proto_deserialize(cursor) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}
