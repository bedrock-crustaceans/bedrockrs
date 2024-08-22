use std::borrow::Cow;
use std::marker::PhantomData;

use paste::paste;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{de, Deserialize};

use crate::error::StreamError;
use crate::read::BinaryRead;
use crate::{BigEndian, FieldType, LittleEndian, NbtError, Variable, Variant, VariantImpl};

/// Verifies that the deserialised type is equal to the expected type.
macro_rules! is_ty {
    ($expected: ident, $actual: expr) => {
        if $actual != FieldType::$expected {
            return Err(NbtError::UnexpectedType {
                expected: FieldType::$expected,
                actual: $actual,
            });
        }
    };
}

/// Returns a `not supported` error.
macro_rules! forward_unsupported {
    ($($ty: ident),+) => {
        paste! {$(
            #[inline]
            fn [<deserialize_ $ty>]<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>
            {
                return Err(NbtError::Unsupported(
                    concat!("Deserialization of `", stringify!($ty), "` is not supported")
                ));
            }
        )+}
    }
}

/// NBT deserialiser.
#[derive(Debug)]
pub struct Deserializer<'re, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl + 'de,
{
    input: &'re mut R,
    next_ty: FieldType,
    is_key: bool,
    _marker: PhantomData<&'de F>,
}

impl<'re, 'de, F, R> Deserializer<'re, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl + 'de,
{
    /// Creates a new deserialiser, consuming the reader.
    pub fn new(input: &'re mut R) -> Result<Self, NbtError> {
        let next_ty = FieldType::try_from(input.read_u8()?)?;
        if next_ty != FieldType::Compound && next_ty != FieldType::List {
            return Err(NbtError::Other(Cow::Borrowed(
                "Expected compound or list tag as root",
            )));
        }

        let mut de = Deserializer {
            input,
            next_ty,
            is_key: false,
            _marker: PhantomData,
        };

        let _: &str = de.deserialize_raw_str()?;

        Ok(de)
    }

    /// Deserialise a raw UTF-8 string.
    fn deserialize_raw_str(&mut self) -> Result<&str, StreamError> {
        let len = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_u16_be()? as u32,
            Variant::LittleEndian => self.input.read_u16_le()? as u32,
            Variant::Variable => self.input.read_var_u32()?,
        };

        let data = self.input.take_n(len as usize)?;
        let str = std::str::from_utf8(data)?;

        Ok(str)
    }
}

/// Reads a single object of type `T` from the given buffer.
///
/// On success, the deserialised object and amount of bytes read from the buffer are returned.
#[inline]
fn from_bytes<'de, 're, F, R, T>(reader: &'re mut R) -> Result<T, NbtError>
where
    R: BinaryRead<'de>,
    T: Deserialize<'de>,
    F: VariantImpl + 'de,
{
    let mut deserializer = Deserializer::<F, R>::new(reader)?;
    let output = T::deserialize(&mut deserializer)?;

    Ok(output)
}

/// Reads a single object of type `T` from the given buffer.
///
/// This function uses the little endian format of NBT, which is used by disk formats
/// in Minecraft: Bedrock Edition.
///
/// On success, the deserialised object and amount of bytes read from the buffer are returned.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize, Debug)]
///  struct Data {
///     value: String
///  }
///
/// # let data = Data {
/// #   value: String::from("Hello, World!")
/// # };
/// # let obuffer = nbt::to_le_bytes(&data).unwrap();
/// # let mut buffer: &[u8] = obuffer.as_ref();
///
///  let result = nbt::from_le_bytes(&mut buffer).unwrap();
///  let data: Data = result.0;
///
///  println!("Got {data:?}!");
/// # }
/// ```
#[inline]
pub fn from_le_bytes<'de, T, R>(reader: &mut R) -> Result<T, NbtError>
where
    R: BinaryRead<'de>,
    T: Deserialize<'de>,
{
    from_bytes::<LittleEndian, _, _>(reader)
}

/// Reads a single object of type `T` from the given buffer.
///
/// This function uses the little endian format of NBT, which is used by
/// Minecraft: Java Edition.
///
/// On success, the deserialised object and amount of bytes read from the buffer are returned.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize, Debug)]
///  struct Data {
///     value: String
///  }
///
/// # let data = Data {
/// #   value: String::from("Hello, World!")
/// # };
/// # let owned_buffer = nbt::to_be_bytes(&data).unwrap();
/// # let mut buffer = owned_buffer.as_slice();
///
///  let result = nbt::from_be_bytes(&mut buffer).unwrap();
///  let data: Data = result.0;
///
///  println!("Got {data:?}!");
/// # }
/// ```
#[inline]
pub fn from_be_bytes<'de, T, R>(reader: &mut R) -> Result<T, NbtError>
where
    R: BinaryRead<'de>,
    T: Deserialize<'de>,
{
    from_bytes::<BigEndian, _, _>(reader)
}

/// Reads a single object of type `T` from the given buffer.
///
/// This function uses the variable format of NBT, which is used by network formats
/// in Minecraft: Bedrock Edition.
///
/// On success, the deserialised object and amount of bytes read from the buffer are returned.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize, Debug)]
///  struct Data {
///     value: String
///  }
///
/// # let data = Data {
/// #   value: String::from("Hello, World!")
/// # };
/// # let owned_buffer = nbt::to_var_bytes(&data).unwrap();
/// # let mut buffer = owned_buffer.as_slice();
///
///  let result = nbt::from_var_bytes(&mut buffer).unwrap();
///  let data: Data = result.0;
///
///  println!("Got {data:?}!");
/// # }
/// ```
#[inline]
pub fn from_var_bytes<'data, T, R>(reader: &mut R) -> Result<T, NbtError>
where
    R: BinaryRead<'data>,
    T: Deserialize<'data>,
{
    from_bytes::<Variable, _, _>(reader)
}

impl<'de, 're, 'a, F, R> de::Deserializer<'de> for &'a mut Deserializer<'re, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl + 'a,
{
    type Error = NbtError;

    forward_unsupported!(char, u8, u16, u32, u64, i128, u128);

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        if self.is_key {
            self.deserialize_str(visitor)
        } else {
            match self.next_ty {
                FieldType::End => {
                    return Err(NbtError::Other(Cow::Borrowed(
                        "Encountered unmatched end tag",
                    )))
                }
                FieldType::Byte => self.deserialize_i8(visitor),
                FieldType::Short => self.deserialize_i16(visitor),
                FieldType::Int => self.deserialize_i32(visitor),
                FieldType::Long => self.deserialize_i64(visitor),
                FieldType::Float => self.deserialize_f32(visitor),
                FieldType::Double => self.deserialize_f64(visitor),
                FieldType::ByteArray => self.deserialize_byte_buf(visitor),
                FieldType::String => self.deserialize_string(visitor),
                FieldType::List => self.deserialize_seq(visitor),
                FieldType::Compound => {
                    let m = self.deserialize_map(visitor);
                    m
                }
                FieldType::IntArray => self.deserialize_seq(visitor),
                FieldType::LongArray => self.deserialize_seq(visitor),
            }
        }
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Byte, self.next_ty);

        let n = self.input.read_bool()?;
        visitor.visit_bool(n)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Byte, self.next_ty);

        let n = self.input.read_i8()?;
        visitor.visit_i8(n)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Short, self.next_ty);

        let n = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_i16_be(),
            Variant::LittleEndian | Variant::Variable => self.input.read_i16_le(),
        }?;

        visitor.visit_i16(n)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Int, self.next_ty);

        let n = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_i32_be(),
            Variant::LittleEndian => self.input.read_i32_le(),
            Variant::Variable => self.input.read_var_i32(),
        }?;

        visitor.visit_i32(n)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Long, self.next_ty);

        let n = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_i64_be(),
            Variant::LittleEndian => self.input.read_i64_le(),
            Variant::Variable => self.input.read_var_i64(),
        }?;

        visitor.visit_i64(n)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Float, self.next_ty);

        let n = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_f32_be(),
            _ => self.input.read_f32_le(),
        }?;

        visitor.visit_f32(n)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Double, self.next_ty);

        let n = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_f64_be(),
            _ => self.input.read_f64_le(),
        }?;

        visitor.visit_f64(n)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        let len = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_u16_be()? as u32,
            Variant::LittleEndian => self.input.read_u16_le()? as u32,
            Variant::Variable => self.input.read_var_u32()?,
        };

        let data = self.input.take_n(len as usize)?;
        let str = std::str::from_utf8(data)?;

        // dbg!(str);

        visitor.visit_str(str)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(String, self.next_ty);

        let len = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_u16_be()? as u32,
            Variant::LittleEndian => self.input.read_u16_le()? as u32,
            Variant::Variable => self.input.read_var_u32()?,
        };

        let data = self.input.take_n(len as usize)?;
        let string = String::from_utf8(data.to_vec())?;

        // dbg!(&string);

        visitor.visit_string(string)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(ByteArray, self.next_ty);

        let len = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_i32_be()? as u32,
            Variant::LittleEndian => self.input.read_i32_le()? as u32,
            Variant::Variable => self.input.read_var_i32()? as u32,
        };

        let buf = self.input.take_n(len as usize)?;
        visitor.visit_bytes(buf)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(ByteArray, self.next_ty);

        let len = match F::AS_ENUM {
            Variant::BigEndian => self.input.read_i32_be()? as u32,
            Variant::LittleEndian => self.input.read_i32_le()? as u32,
            Variant::Variable => self.input.read_var_i32()? as u32,
        };

        let buf = self.input.take_n(len as usize)?.to_vec();
        visitor.visit_byte_buf(buf)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        // This is only used to represent possibly missing fields.
        // If this code is reached, it means the key was found and the field exists.
        // Therefore this is always some.
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        Err(NbtError::Unsupported(
            "Deserializing unit values is not supported",
        ))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        Err(NbtError::Unsupported(
            "Deserializing unit structs is not supported",
        ))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        Err(NbtError::Unsupported(
            "Deserializing newtype structs is not supported",
        ))
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(0, visitor)
    }

    #[inline]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        let ty = match self.next_ty {
            FieldType::ByteArray => FieldType::Byte,
            FieldType::IntArray => FieldType::Int,
            FieldType::LongArray => FieldType::Long,
            _ => FieldType::try_from(self.input.read_u8()?)?,
        };

        let de = SeqDeserializer::new(self, ty, len as u32)?;
        visitor.visit_seq(de)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        Err(NbtError::Unsupported(
            "Deserializing tuple structs is not supported",
        ))
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        is_ty!(Compound, self.next_ty);

        let de = MapDeserializer::from(self);
        visitor.visit_map(de)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        Err(NbtError::Unsupported(
            "Deserializing enums is not supported",
        ))
    }

    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, NbtError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Deserializes NBT sequences.
///
/// Sequences are in this case: [`ByteArray`](FieldType::ByteArray), [`IntArray`](FieldType::IntArray)
/// [`LongArray`](FieldType::LongArray) and [`List`](FieldType::List).
#[derive(Debug)]
struct SeqDeserializer<'a, 're, 'de: 'a, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    de: &'a mut Deserializer<'re, 'de, F, R>,
    ty: FieldType,
    remaining: u32,
}

impl<'de, 're, 'a, F, R> SeqDeserializer<'a, 're, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    #[inline]
    pub fn new(
        de: &'a mut Deserializer<'re, 'de, F, R>,
        ty: FieldType,
        expected_len: u32,
    ) -> Result<Self, NbtError> {
        // debug_assert_ne!(ty, FieldType::End, "Cannot serialize sequence of end tags");

        // ty is not read in here because the x_array types don't have a type prefix.

        de.next_ty = ty;
        let remaining = match F::AS_ENUM {
            Variant::BigEndian => de.input.read_i32_be()? as u32,
            Variant::LittleEndian => de.input.read_i32_le()? as u32,
            Variant::Variable => de.input.read_var_i32()? as u32,
        };

        if expected_len != 0 && expected_len != remaining {
            return Err(NbtError::Other(Cow::Owned(format!(
                "Sequence of {expected_len} {ty:?} expected, found only {remaining} items"
            ))));
        }

        Ok(Self { de, ty, remaining })
    }
}

impl<'de, 're, 'a, F, R> SeqAccess<'de> for SeqDeserializer<'a, 're, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    type Error = NbtError;

    #[inline]
    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, NbtError>
    where
        E: DeserializeSeed<'de>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;

            let output = seed.deserialize(&mut *self.de).map(Some);
            self.de.next_ty = self.ty;
            output
        } else {
            Ok(None)
        }
    }
}

/// Deserialises NBT compounds.
#[derive(Debug)]
struct MapDeserializer<'a, 're, 'de: 'a, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    de: &'a mut Deserializer<'re, 'de, F, R>,
}

impl<'de, 're, 'a, F, R> From<&'a mut Deserializer<'re, 'de, F, R>>
    for MapDeserializer<'a, 're, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    #[inline]
    fn from(v: &'a mut Deserializer<'re, 'de, F, R>) -> Self {
        Self { de: v }
    }
}

impl<'de, 're, 'a, F, R> MapAccess<'de> for MapDeserializer<'a, 're, 'de, F, R>
where
    R: BinaryRead<'de>,
    F: VariantImpl,
{
    type Error = NbtError;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, NbtError>
    where
        K: DeserializeSeed<'de>,
    {
        self.de.is_key = true;
        self.de.next_ty = FieldType::String;

        let next_ty = FieldType::try_from(self.de.input.read_u8()?)?;

        let r = if next_ty == FieldType::End {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        };

        self.de.is_key = false;
        self.de.next_ty = next_ty;
        r
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, NbtError>
    where
        V: DeserializeSeed<'de>,
    {
        debug_assert_ne!(
            self.de.next_ty,
            FieldType::End,
            "Cannot serialize end as a map field"
        );
        seed.deserialize(&mut *self.de)
    }
}
