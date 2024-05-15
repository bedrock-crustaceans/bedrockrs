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

    pub fn get_id(&self) -> u8 {
        match self {
            NbtTag::Byte(_) => { Self::BYTE_ID }
            NbtTag::Int16(_) => { Self::INT16_ID }
            NbtTag::Int32(_) => { Self::INT32_ID }
            NbtTag::Int64(_) => { Self::INT64_ID }
            NbtTag::Float32(_) => { Self::FLOAT32_ID }
            NbtTag::Float64(_) => { Self::FLOAT64_ID }
            NbtTag::String(_) => { Self::STRING_ID }
            NbtTag::List(_) => { Self::LIST_ID }
            NbtTag::Compound(_) => { Self::COMPOUND_ID_START }
        }
    }

    pub fn nbt_serialize<T: NbtByteOrder>(
        &self,
        key: impl Into<String>,
        buf: &mut Vec<u8>,
    ) -> Result<(), NbtError> {
        match T::write_u8(buf, self.get_id()) {
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

        self.nbt_serialize_val::<T>(buf)
    }

    fn nbt_serialize_val<T: NbtByteOrder>(
        &self,
        buf: &mut Vec<u8>,
    ) -> Result<(), NbtError> {
        match self {
            NbtTag::Byte(v) => {
                match T::write_u8(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int16(v) => {
                match T::write_i16(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int32(v) => {
                match T::write_i32(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Int64(v) => {
                match T::write_i64(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Float32(v) => {
                match T::write_f32(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Float64(v) => {
                match T::write_f64(buf, *v) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::String(v) => {
                match T::write_string(buf, v.to_string()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::List(v) => {
                let list_type = if v.is_empty() {
                    Self::BYTE_ID
                } else {
                    v[0].get_id()
                };

                match T::write_u8(buf, list_type) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                match T::write_i32(buf, match v.len().try_into() {
                    Ok(v) => { v }
                    Err(e) => { return Err(NbtError::IntError(e)) }
                }) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                for tag in v {
                    match tag.nbt_serialize_val::<T>(buf) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            NbtTag::Compound(v) => {
                let iter = v.iter();

                for (key, v) in iter {
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

        if id == Self::COMPOUND_ID_END {
            return Err(NbtError::CompoundClosingTag)
        }

        let name = match T::read_string(cursor) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let tag = match Self::nbt_deserialize_val::<T>(cursor, id) {
            Ok(v) => { v },
            Err(e) => { return Err(e) }
        };

        Ok((name, tag))
    }

    fn nbt_deserialize_val<T: NbtByteOrder>(
        cursor: &mut Cursor<Vec<u8>>,
        id: u8
    ) -> Result<Self, NbtError> {
        let tag = match id {
            Self::BYTE_ID => {
                let byte = match T::read_u8(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Byte(byte)
            }
            Self::INT16_ID => {
                let int16 = match T::read_i16(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int16(int16)
            }
            Self::INT32_ID => {
                let int32 = match T::read_i32(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int32(int32)
            }
            Self::INT64_ID => {
                let int64 = match T::read_i64(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int64(int64)
            }
            Self::FLOAT32_ID => {
                let float32 = match T::read_f32(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Float32(float32)
            }
            Self::FLOAT64_ID => {
                let float64 = match T::read_f64(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Float64(float64)
            }
            Self::STRING_ID => {
                let string = match T::read_string(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::String(string)
            }
            Self::LIST_ID => {
                let list_type = match T::read_u8(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let len = match T::read_i32(cursor) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let mut vec = vec![];

                for i in 0..(len-1) {
                    match Self::nbt_deserialize_val::<T>(cursor, list_type) {
                        Ok(v) => { vec.push(v) }
                        Err(e) => { return Err(e) }
                    }
                }

                NbtTag::List(vec)
            }
            Self::COMPOUND_ID_START => {
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

                NbtTag::Compound(map)
            }
            Self::COMPOUND_ID_END => {
                return Err(NbtError::CompoundClosingTag);
            }
            other => {
                return Err(NbtError::UnexpectedID(other));
            }
        };

        Ok(tag)
    }
}
