use std::collections::HashMap;
use std::io::Cursor;

use crate::byte_order::NbtByteOrder;
use crate::error::NbtError;

pub mod byte_order;
pub mod endian;
pub mod error;

#[derive(Debug)]
pub enum NbtTag {
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
}

impl NbtTag {
    const BYTE_ID: u8 = 0x01;
    const INT16_ID: u8 = 0x02;
    const INT32_ID: u8 = 0x03;
    const INT64_ID: u8 = 0x04;
    const FLOAT32_ID: u8 = 0x05;
    const FLOAT64_ID: u8 = 0x06;
    const STRING_ID: u8 = 0x08;
    const LIST_ID: u8 = 0x09;
    const COMPOUND_ID_START: u8 = 0x0A;
    const COMPOUND_ID_END: u8 = 0x00;

    pub fn nbt_serialize<T: NbtByteOrder>(
        &self,
        key: impl Into<String>,
        buf: &mut Vec<u8>,
    ) -> Result<(), NbtError> {
        match self {
            NbtTag::Byte(v) => {
                match T::write_u8(buf, Self::BYTE_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_u8(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int16(v) => {
                match T::write_u8(buf, Self::INT16_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_i16(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int32(v) => {
                match T::write_u8(buf, Self::INT32_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_i32(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int64(v) => {
                match T::write_u8(buf, Self::INT64_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_i64(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Float32(v) => {
                match T::write_u8(buf, Self::FLOAT32_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_f32(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Float64(v) => {
                match T::write_u8(buf, Self::FLOAT64_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_f64(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::String(v) => {
                match T::write_u8(buf, Self::STRING_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, v.to_string()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::List(_) => {
                todo!()
            }
            NbtTag::Compound(v) => {
                match T::write_u8(buf, Self::COMPOUND_ID_START) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_string(buf, key.into()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                for (key, v) in v {
                    match v.nbt_serialize::<T>(key, buf) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                match T::write_u8(buf, Self::COMPOUND_ID_END) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn nbt_deserialize<T: NbtByteOrder>(
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<(String, Self), NbtError> {
        let id = match T::read_u8(cursor) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let (name, tag) = match id {
            Self::BYTE_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let byte = match T::read_u8(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Byte(byte))
            }
            Self::INT16_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let int16 = match T::read_i16(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Int16(int16))
            }
            Self::INT32_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let int32 = match T::read_i32(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Int32(int32))
            }
            Self::INT64_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let int64 = match T::read_i64(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Int64(int64))
            }
            Self::FLOAT32_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let float32 = match T::read_f32(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Float32(float32))
            }
            Self::FLOAT64_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let float64 = match T::read_f64(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::Float64(float64))
            }
            Self::STRING_ID => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let string = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                (name, NbtTag::String(string))
            }
            Self::LIST_ID => {
                todo!()
            }
            Self::COMPOUND_ID_START => {
                let name = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let mut map = HashMap::new();

                'compound_loop: loop {
                    let (key, tag) = match Self::nbt_deserialize::<T>(cursor) {
                        Ok(v) => v,
                        Err(e) => match e {
                            NbtError::CompoundClosingTag => {
                                break 'compound_loop;
                            }
                            other => {
                                return Err(other);
                            }
                        },
                    };

                    map.insert(key, tag);
                }

                (name, NbtTag::Compound(map))
            }
            Self::COMPOUND_ID_END => {
                return Err(NbtError::CompoundClosingTag);
            }
            other => {
                return Err(NbtError::UnexpectedID(other));
            }
        };

        Ok((name, tag))
    }
}
