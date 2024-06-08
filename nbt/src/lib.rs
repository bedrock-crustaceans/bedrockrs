use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use bedrock_core::stream::read::ByteStreamRead;
pub use error::NbtError;

use crate::byte_order::NbtByteOrder;

pub mod byte_order;
pub mod endian;
pub mod error;

/// An enum representing all possible NBT tags.
///
/// Still missing ([GitHub issue](https://github.com/Adrian8115/bedrock-rs/issues/8)):
/// - byte array
/// - int array
/// - long array
///
/// (These missing types are rarely, if not even un-, used in MCBE)
#[derive(Clone)]
pub enum NbtTag {
    /// A simple byte.
    /// Can represent multiple things like:
    /// - a boolean
    /// - an enum
    /// - an 8-bit (possibly signed/unsigned) integer
    /// - ...
    Byte(u8),
    /// A 16-bit signed integer
    Int16(i16),
    /// A 32-bit signed integer
    Int32(i32),
    /// A 64-bit signed integer
    Int64(i64),
    /// A 32-bit float
    Float32(f32),
    /// A 64-bit float
    Float64(f64),
    /// A simple string
    /// Should never be larger than [i16::MAX]
    String(String),
    /// A list of NBTs.
    ///
    /// All elements should use the same enum variant.
    /// The serialized type declaration is the first element in the list
    List(Vec<NbtTag>),
    /// A key-value pair map of NBTs.
    ///
    /// Each key is a String with the following NBT tag.
    /// Tag enum variants may vary.
    /// Compound tags are special because they are opened by the [NbtTag::COMPOUND_ID]
    /// and closed again by the [NbtTag::EMPTY_ID].
    Compound(HashMap<String, NbtTag>),
    /// An empty NBT tag.
    /// Not commonly used, it rather just marks the end of a compound tag.
    Empty,
}

impl NbtTag {
    /// The tag ID of [NbtTag::Byte]
    const BYTE_ID: u8 = 0x01;

    /// The tag ID of [NbtTag::Int16]
    const INT16_ID: u8 = 0x02;

    /// The tag ID of [NbtTag::Int32]
    const INT32_ID: u8 = 0x03;

    /// The tag ID of [NbtTag::Int64]
    const INT64_ID: u8 = 0x04;

    /// The tag ID of [NbtTag::Float32]
    const FLOAT32_ID: u8 = 0x05;

    /// The tag ID of [NbtTag::Float64]
    const FLOAT64_ID: u8 = 0x06;

    /// The tag ID of [NbtTag::String]
    const STRING_ID: u8 = 0x08;

    /// The tag ID of [NbtTag::List]
    const LIST_ID: u8 = 0x09;

    /// The open tag ID of [NbtTag::Compound]
    const COMPOUND_ID: u8 = 0x0A;

    /// The close tag ID of [NbtTag::Compound]
    const EMPTY_ID: u8 = 0x00;

    /// Returns the tag (open) ID for a given tag.
    #[inline(always)]
    fn get_id(&self) -> u8 {
        match self {
            NbtTag::Byte(_) => Self::BYTE_ID,
            NbtTag::Int16(_) => Self::INT16_ID,
            NbtTag::Int32(_) => Self::INT32_ID,
            NbtTag::Int64(_) => Self::INT64_ID,
            NbtTag::Float32(_) => Self::FLOAT32_ID,
            NbtTag::Float64(_) => Self::FLOAT64_ID,
            NbtTag::String(_) => Self::STRING_ID,
            NbtTag::List(_) => Self::LIST_ID,
            NbtTag::Compound(_) => Self::COMPOUND_ID,
            NbtTag::Empty => Self::EMPTY_ID,
        }
    }

    /// Serializes a given NBT with a given root tag
    /// (the root tags name is nearly always left empty) name into the given buffer.
    /// Use the provided endian for serialization.
    ///
    /// Use [NbtTag::nbt_serialize] as a simple alternative that just returns
    /// a vec as its output.
    ///
    /// # Example:
    /// ```no_run
    /// use std::collections::HashMap;
    /// use nbt::endian::little_endian::NbtLittleEndian;
    /// use nbt::NbtTag;
    ///
    /// let mut map = HashMap::new();
    ///
    /// map.insert("MyText".to_string(), NbtTag::String("This is my text".to_string()));
    /// map.insert("MyInt32".to_string(), NbtTag::Int32(42));
    ///
    /// let tag = NbtTag::Compound(map);
    ///
    /// let mut buf = vec![];
    ///
    /// // You can also use other endian encodings
    /// NbtTag::nbt_serialize::<NbtLittleEndian>(&tag, "", &mut buf).unwrap();
    ///
    /// println!("Nbt: {:#?}", tag);
    /// println!("Raw: {:?}", buf);
    /// ```
    pub fn nbt_serialize<T: NbtByteOrder>(
        &self,
        root_tag_name: impl Into<String>,
        buf: &mut Vec<u8>,
    ) -> Result<(), NbtError> {
        match T::write_u8(buf, self.get_id()) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match T::write_string(buf, root_tag_name.into()) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        self.nbt_serialize_val::<T>(buf)
    }

    /// Serialize the NBT via a simple vec.
    /// Simpler alternative to [NbtTag::nbt_serialize].
    #[inline]
    pub fn nbt_serialize_vec<T: NbtByteOrder>(
        &self,
        key: impl Into<String>,
    ) -> Result<Vec<u8>, NbtError> {
        let mut buf = vec![];

        match NbtTag::nbt_serialize::<T>(self, key, &mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    /// Serializes a given val without any root tag name or type notation.
    /// Should only be used by the [NbtTag::nbt_serialize] function internally.
    #[inline]
    fn nbt_serialize_val<T: NbtByteOrder>(&self, buf: &mut Vec<u8>) -> Result<(), NbtError> {
        match self {
            NbtTag::Byte(v) => match T::write_u8(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::Int16(v) => match T::write_i16(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::Int32(v) => match T::write_i32(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::Int64(v) => match T::write_i64(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::Float32(v) => match T::write_f32(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::Float64(v) => match T::write_f64(buf, *v) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            NbtTag::String(v) => match T::write_string(buf, v.to_string()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
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

                match T::write_i32(
                    buf,
                    match v.len().try_into() {
                        Ok(v) => v,
                        Err(e) => return Err(NbtError::IntError(e)),
                    },
                ) {
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

                for (tag_name, v) in iter {
                    match v.nbt_serialize::<T>(tag_name, buf) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                match T::write_u8(buf, Self::EMPTY_ID) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            NbtTag::Empty => {}
        }

        Ok(())
    }

    /// Deserializes an NBT from a [`ByteStreamRead`], returns the NBT and its root tag name
    /// (The root tag name is nearly always left empty).
    /// Uses the provided endian encoding for deserialization.
    ///
    /// Use [`NbtTag::nbt_deserialize_vec`] as a simple alternative that just takes
    /// a vec as an argument.
    ///
    /// # Example:
    /// ```no_run
    /// use std::io::Cursor;
    /// use bedrock_core::stream::read::ByteStreamRead;
    /// use nbt::endian::little_endian::NbtLittleEndian;
    /// use nbt::NbtTag;
    ///
    /// // Data generated by the NbtTag::serialize function's example
    /// let data = vec![
    ///     10, 6, 0, 109, 121, 32, 110, 98, 116,
    ///     8, 7, 0, 77, 121, 32, 84, 101, 120, 116,
    ///     15, 0, 84, 104, 105, 115, 32, 105, 115,
    ///     32, 109, 121, 32, 116, 101, 120, 116, 3,
    ///     8, 0, 77, 121, 32, 105, 110, 116, 51, 50,
    ///     42, 0, 0, 0, 0
    /// ];
    ///
    /// // Create the stream
    /// let mut stream = ByteStreamRead::from(data);
    ///
    /// // Read the nbt tag and its name
    /// let (tag, name) = NbtTag::nbt_deserialize::<NbtLittleEndian>(&mut stream).unwrap();
    ///
    /// println!("{:#?}: {:#?}", name, tag);
    /// ```
    pub fn nbt_deserialize<T: NbtByteOrder>(
        stream: &mut ByteStreamRead,
    ) -> Result<(String, Self), NbtError> {
        let id = match T::read_u8(stream) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let name = match T::read_string(stream) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let tag = match Self::nbt_deserialize_val::<T>(stream, id) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok((name, tag))
    }

    /// Deserialize the NBT via a simple vec.
    /// Simpler alternative to [NbtTag::nbt_deserialize].
    #[inline]
    pub fn nbt_deserialize_vec<T: NbtByteOrder>(vec: &Vec<u8>) -> Result<(String, Self), NbtError> {
        NbtTag::nbt_deserialize::<T>(&mut ByteStreamRead::from(vec.clone()))
    }

    /// Deserializes a given val without reading any tag name notation.
    /// Should only be used by the [NbtTag::nbt_deserialize] function internally.
    #[inline]
    fn nbt_deserialize_val<T: NbtByteOrder>(
        stream: &mut ByteStreamRead,
        id: u8,
    ) -> Result<Self, NbtError> {
        let tag = match id {
            Self::BYTE_ID => {
                let byte = match T::read_u8(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Byte(byte)
            }
            Self::INT16_ID => {
                let int16 = match T::read_i16(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int16(int16)
            }
            Self::INT32_ID => {
                let int32 = match T::read_i32(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int32(int32)
            }
            Self::INT64_ID => {
                let int64 = match T::read_i64(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Int64(int64)
            }
            Self::FLOAT32_ID => {
                let float32 = match T::read_f32(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Float32(float32)
            }
            Self::FLOAT64_ID => {
                let float64 = match T::read_f64(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::Float64(float64)
            }
            Self::STRING_ID => {
                let string = match T::read_string(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                NbtTag::String(string)
            }
            Self::LIST_ID => {
                let list_type = match T::read_u8(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let len = match T::read_i32(stream) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let mut vec = vec![];

                for _ in 0..len {
                    match Self::nbt_deserialize_val::<T>(stream, list_type) {
                        Ok(v) => vec.push(v),
                        Err(e) => return Err(e),
                    }
                }

                NbtTag::List(vec)
            }
            Self::COMPOUND_ID => {
                let mut map = HashMap::new();

                loop {
                    let id = match T::read_u8(stream) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    if id == Self::EMPTY_ID {
                        break;
                    }

                    let key = match T::read_string(stream) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    let tag = match Self::nbt_deserialize_val::<T>(stream, id) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    map.insert(key, tag);
                }

                NbtTag::Compound(map)
            }
            Self::EMPTY_ID => NbtTag::Empty,
            other => {
                return Err(NbtError::UnexpectedID(other));
            }
        };

        Ok(tag)
    }
}

// Implement the Debug trait for NbtTag
impl Debug for NbtTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // check for pretty formating with #
        match f.alternate() {
            // normal format
            false => match self {
                NbtTag::Byte(v) => {
                    write!(f, "Byte({v:?})")
                }
                NbtTag::Int16(v) => {
                    write!(f, "Int16({v:?})")
                }
                NbtTag::Int32(v) => {
                    write!(f, "Int32({v:?})")
                }
                NbtTag::Int64(v) => {
                    write!(f, "Int64({v:?})")
                }
                NbtTag::Float32(v) => {
                    write!(f, "Float32({v:?})")
                }
                NbtTag::Float64(v) => {
                    write!(f, "Float64({v:?})")
                }
                NbtTag::String(v) => {
                    write!(f, "{v:?}")
                }
                NbtTag::List(v) => {
                    write!(f, "{v:?}")
                }
                NbtTag::Compound(v) => {
                    write!(f, "{v:?}")
                }
                NbtTag::Empty => {
                    write!(f, "")
                }
            },
            // pretty format
            true => match self {
                NbtTag::Byte(v) => {
                    write!(f, "Byte({v:#?})")
                }
                NbtTag::Int16(v) => {
                    write!(f, "Int16({v:#?})")
                }
                NbtTag::Int32(v) => {
                    write!(f, "Int32({v:#?})")
                }
                NbtTag::Int64(v) => {
                    write!(f, "Int64({v:#?})")
                }
                NbtTag::Float32(v) => {
                    write!(f, "Float32({v:#?})")
                }
                NbtTag::Float64(v) => {
                    write!(f, "Float64({v:#?})")
                }
                NbtTag::String(v) => {
                    write!(f, "{v:#?}")
                }
                NbtTag::List(v) => {
                    write!(f, "{v:#?}")
                }
                NbtTag::Compound(v) => {
                    write!(f, "{v:#?}")
                }
                // Any better idea of what should be written?
                NbtTag::Empty => {
                    write!(f, "EMPTY")
                }
            },
        }
    }
}
