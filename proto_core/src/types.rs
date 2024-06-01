use std::f32;
use std::io::{Read, Write};

use bedrock_core::*;
use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

// i8
impl ProtoCodec for i8 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
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
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u8(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
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
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i16le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i16le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i16be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i16be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i16be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u16
impl ProtoCodec for u16le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u16le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u16le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u16be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u16be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u16be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i32

impl ProtoCodec for i32le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i32le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i32le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i32be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i32be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i32be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u32

impl ProtoCodec for u32le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u32le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u32le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u32be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u32be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u32be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i64

impl ProtoCodec for i64le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i64le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i64le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i64be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i64be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i64be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u64

impl ProtoCodec for u64le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u64le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u64le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// i128

impl ProtoCodec for i128le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i128le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i128le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for i128be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_i128be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_i128be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// u128

impl ProtoCodec for u128le {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u128le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u128le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

impl ProtoCodec for u128be {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_u128be(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_u128be() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// f32

impl ProtoCodec for f32 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_f32le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_f32le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// f64

impl ProtoCodec for f64 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_f64le(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_f64le() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// uvarint 32

impl ProtoCodec for uvar32 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_uvar32(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_uvar32() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// ivarint 32

impl ProtoCodec for ivar32 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_ivar32(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_ivar32() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// uvarint 64

impl ProtoCodec for uvar64 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_uvar64(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_uvar64() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// ivarint 64

impl ProtoCodec for ivar64 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_ivar64(*self) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }

    fn proto_deserialize(data: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match data.read_ivar64() {
            Ok(v) => Ok(v),
            Err(e) => Err(ProtoCodecError::IOError(e)),
        }
    }
}

// bool

impl ProtoCodec for bool {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        // a bool is represented as a byte
        return match stream.read_u8() {
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
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_uvar32(uvar32(self.len() as u32)) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        match buf.write_all(self.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match stream.read_uvar32() {
            Ok(v) => v.0,
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        };

        let mut string_buf = vec![0u8; len as usize];

        match stream.read_exact(&mut *string_buf) {
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
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match buf.write_uvar32(uvar32(self.len() as u32)) {
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match stream.read_uvar32() {
            Ok(v) => v.0,
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        };

        let mut array = vec![];

        for _ in 0..len {
            array.push(match T::proto_deserialize(stream) {
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
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match bool::proto_deserialize(stream) {
            Ok(v) => match v {
                false => Ok(Option::None),
                true => match T::proto_deserialize(stream) {
                    Ok(v) => Ok(Option::Some(v)),
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(e),
        }
    }
}

impl ProtoCodec for Vec2 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match i32le::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match i32le::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec2f {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match f32::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match f32::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3 {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match i32le::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            y: match i32le::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match i32le::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3f {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match f32::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            y: match f32::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
            z: match f32::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}
