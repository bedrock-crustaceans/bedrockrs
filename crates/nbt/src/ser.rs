use std::marker::PhantomData;

use paste::paste;
use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple};
use serde::{ser, Serialize};

use util::{BinaryWrite, RVec};

use crate::{BigEndian, FieldType, LittleEndian, NbtError, Variable, Variant, VariantImpl};

/// Returns a `not supported` error.
macro_rules! forward_unsupported {
    ($($ty: ident),+) => {
        paste! {$(
           #[inline]
            fn [<serialize_ $ty>](self, _v: $ty) -> Result<(), NbtError> {
               Err(anyhow::anyhow!(concat!("Serialisation of `", stringify!($ty), "` is not supported")).into())
            }
        )+}
    }
}

/// Returns a `not supported` error.
macro_rules! forward_unsupported_field {
    ($($ty: ident),+) => {
        paste! {$(
           #[inline]
            fn [<serialize_ $ty>](self, _v: $ty) -> Result<bool, NbtError> {
               Err(anyhow::anyhow!(concat!("Serialisation of `", stringify!($ty), "` is not supported")).into())
            }
        )+}
    }
}

/// Serializes the given data in big endian format.
///
/// This is the format used by Minecraft: Java Edition.
///
/// See [`to_be_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbt::to_be_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_be_bytes<T>(v: &T) -> anyhow::Result<RVec>
where
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::<_, BigEndian>::new(RVec::alloc());

    v.serialize(&mut ser)?;
    Ok(ser.into_inner())
}

/// Serializes the given data in little endian format.
///
/// This is the format used by disk formats in Minecraft: Bedrock Edition.
///
/// See [`to_le_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbt::to_le_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_le_bytes<T>(v: &T) -> anyhow::Result<RVec>
where
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::<_, LittleEndian>::new(RVec::alloc());

    v.serialize(&mut ser)?;
    Ok(ser.into_inner())
}

/// Serializes the given data in variable format.
///
/// This is the format used by network formats in Minecraft: Bedrock Edition.
///
/// See [`to_var_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbt::to_var_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_var_bytes<T>(v: &T) -> anyhow::Result<RVec>
where
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::<_, Variable>::new(RVec::alloc());

    v.serialize(&mut ser)?;
    Ok(ser.into_inner())
}

/// Serializes the given data, into the given writer, in big endian format.
///
/// This is the format used by Minecraft: Java Edition.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Vec::new();
///
///  nbt::to_be_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_be_bytes_in<W, T>(w: W, v: &T) -> anyhow::Result<()>
where
    T: ?Sized + Serialize,
    W: BinaryWrite,
{
    let mut ser = Serializer::<W, BigEndian>::new(w);
    v.serialize(&mut ser)?;

    Ok(())
}

/// Serializes the given data, into the given writer, in little endian format.
///
/// This is the format used by disk formats in Minecraft: Bedrock Edition.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Vec::new();
///
///  nbt::to_le_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_le_bytes_in<W, T>(w: W, v: &T) -> anyhow::Result<()>
where
    T: ?Sized + Serialize,
    W: BinaryWrite,
{
    let mut ser = Serializer::<W, LittleEndian>::new(w);
    v.serialize(&mut ser)?;

    Ok(())
}

/// Serializes the given data, into the given writer, in variable format.
///
/// This is the format used by network formats in Minecraft: Bedrock Edition.
///
/// # Example
///
/// ```rust
/// # use mirai_nbt as nbt;
/// #
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Vec::new();
///
///  nbt::to_var_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_var_bytes_in<W, T>(w: W, value: &T) -> anyhow::Result<()>
where
    T: ?Sized + Serialize,
    W: BinaryWrite,
{
    let mut ser = Serializer::<W, Variable>::new(w);
    value.serialize(&mut ser)?;

    Ok(())
}

/// NBT data serialiser.
#[derive(Debug)]
pub struct Serializer<W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    writer: W,
    /// Whether this is the first data to be written.
    /// This makes sure that the name and type of the root compound are written.
    is_initial: bool,
    /// Stores the length of the list that is currently being serialised.
    len: usize,
    _marker: PhantomData<F>,
}

impl<W, M> Serializer<W, M>
where
    W: BinaryWrite,
    M: VariantImpl,
{
    /// Creates a new and empty serialiser.
    #[inline]
    pub const fn new(w: W) -> Serializer<W, M> {
        Serializer {
            writer: w,
            is_initial: true,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Consumes the serialiser and returns the inner writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W, M> ser::Serializer for &'a mut Serializer<W, M>
where
    M: VariantImpl,
    W: BinaryWrite,
{
    type Ok = ();
    type Error = NbtError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<(), NbtError>;
    type SerializeTupleVariant = Impossible<(), NbtError>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), NbtError>;

    forward_unsupported!(char, u8, u16, u32, u64, i128);

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<(), NbtError> {
        self.writer.write_bool(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<(), NbtError> {
        self.writer.write_i8(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_i16_be(v)?,
            Variant::LittleEndian | Variant::Variable => self.writer.write_i16_le(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_i32_be(v)?,
            Variant::LittleEndian => self.writer.write_i32_le(v)?,
            Variant::Variable => self.writer.write_var_i32(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_i64_be(v)?,
            Variant::LittleEndian => self.writer.write_i64_le(v)?,
            Variant::Variable => self.writer.write_var_i64(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_f32_be(v)?,
            Variant::LittleEndian | Variant::Variable => self.writer.write_f32_le(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_f64_be(v)?,
            Variant::LittleEndian | Variant::Variable => self.writer.write_f64_le(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_u16_be(v.len() as u16),
            Variant::LittleEndian => self.writer.write_u16_le(v.len() as u16),
            Variant::Variable => self.writer.write_var_u32(v.len() as u32),
        }?;

        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<(), NbtError> {
        match M::AS_ENUM {
            Variant::BigEndian => self.writer.write_i32_be(v.len() as i32),
            Variant::LittleEndian => self.writer.write_i32_le(v.len() as i32),
            Variant::Variable => self.writer.write_var_i32(v.len() as i32),
        }?;

        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing None is not supported").into())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), NbtError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing unit is not supported").into())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing unit structs is not supported").into())
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing unit variants is not supported").into())
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _name: &'static str, _value: &T) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing newtype structs is not supported").into())
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), NbtError> {
        Err(anyhow::anyhow!("Serializing newtype variants is not supported").into())
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.len = len;
            Ok(self)
        } else {
            Err(anyhow::anyhow!("Sequences with a size not known upfront are not supported").into())
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.len = len;
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(anyhow::anyhow!("Serializing tuple structs is not supported").into())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(anyhow::anyhow!("Serializing tuple variants is not supported").into())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // nbt::Value does not distinguish between maps and structs.
        // Therefore this is also needed here
        if self.is_initial {
            self.writer.write_u8(FieldType::Compound as u8)?;
            self.serialize_str("")?;
            self.is_initial = false;
        }

        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        if self.is_initial {
            self.writer.write_u8(FieldType::Compound as u8)?;
            self.serialize_str(name)?;
            self.is_initial = false;
        }

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(anyhow::anyhow!("Serializing struct variants is not supported").into())
    }
}

impl<'a, W, F> SerializeSeq for &'a mut Serializer<W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = ();
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        if self.len != 0 {
            let ty_serializer = FieldTypeSerializer::new(self);
            element.serialize(ty_serializer)?;

            match F::AS_ENUM {
                Variant::BigEndian => self.writer.write_i32_be(self.len as i32),
                Variant::LittleEndian => self.writer.write_i32_le(self.len as i32),
                Variant::Variable => self.writer.write_var_i32(self.len as i32),
            }?;
            self.len = 0;
        }

        element.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        Ok(())
    }
}

impl<'a, W, M> SerializeTuple for &'a mut Serializer<W, M>
where
    W: BinaryWrite,
    M: VariantImpl,
{
    type Ok = ();
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        if self.len != 0 {
            let ty_serializer = FieldTypeSerializer::new(self);
            element.serialize(ty_serializer)?;

            match M::AS_ENUM {
                Variant::BigEndian => self.writer.write_i32_be(self.len as i32),
                Variant::LittleEndian => self.writer.write_i32_le(self.len as i32),
                Variant::Variable => self.writer.write_var_i32(self.len as i32),
            }?;
            self.len = 0;
        }

        element.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        Ok(())
    }
}

impl<'a, W, M> SerializeMap for &'a mut Serializer<W, M>
where
    W: BinaryWrite,
    M: VariantImpl,
{
    type Ok = ();
    type Error = NbtError;

    fn serialize_key<K>(&mut self, _key: &K) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
    {
        Err(anyhow::anyhow!("Use MapSerializer::serialize_entry instead").into())
    }

    fn serialize_value<V>(&mut self, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Err(anyhow::anyhow!("Use MapSerializer::serialize_entry instead").into())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let ty_serializer = FieldTypeSerializer::new(self);
        value.serialize(ty_serializer)?;

        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        self.writer.write_u8(FieldType::End as u8)?;
        Ok(())
    }
}

impl<'a, W, M> SerializeStruct for &'a mut Serializer<W, M>
where
    W: BinaryWrite,
    M: VariantImpl,
{
    type Ok = ();
    type Error = NbtError;

    fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        let ty_serializer = FieldTypeSerializer::new(self);
        let should_skip = value.serialize(ty_serializer)?;

        if !should_skip {
            match M::AS_ENUM {
                Variant::LittleEndian => self.writer.write_u16_le(key.len() as u16),
                Variant::BigEndian => self.writer.write_u16_be(key.len() as u16),
                Variant::Variable => self.writer.write_var_u32(key.len() as u32),
            }?;

            self.writer.write_all(key.as_bytes())?;
            value.serialize(&mut **self)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        self.writer.write_u8(FieldType::End as u8)?;
        Ok(())
    }
}

/// Separate serialiser that writes data types to the writer.
///
/// Serde does not provide any type information, hence this exists.
///
/// This serialiser writes the data type of the given value and does not consume it.
struct FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    pub fn new(ser: &'a mut Serializer<W, F>) -> Self {
        Self { ser }
    }
}

impl<'a, W, F> ser::Serializer for FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = bool; // Whether the field should be skipped
    type Error = NbtError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<bool, Self::Error>;
    type SerializeTupleVariant = Impossible<bool, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<bool, Self::Error>;

    forward_unsupported_field!(char, u8, u16, u32, u64, i128);

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<bool, Self::Error> {
        self.ser.writer.write_u8(FieldType::Byte as u8)?;
        Ok(false)
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Byte as u8)?;
        Ok(false)
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Short as u8)?;
        Ok(false)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Int as u8)?;
        Ok(false)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Long as u8)?;
        Ok(false)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Float as u8)?;
        Ok(false)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Double as u8)?;
        Ok(false)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::String as u8)?;
        Ok(false)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::ByteArray as u8)?;
        Ok(false)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(true) // Skip field
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)?;
        Ok(false)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(anyhow::anyhow!("Serializing unit is not supported").into())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(anyhow::anyhow!("Serializing unit structs is not supported").into())
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(anyhow::anyhow!("Serializing unit variants is not supported").into())
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(anyhow::anyhow!("Serializing newtype structs is not supported").into())
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(anyhow::anyhow!("Serializing newtype variants is not supported").into())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ser.writer.write_u8(FieldType::List as u8)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ser.writer.write_u8(FieldType::List as u8)?;
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(anyhow::anyhow!("Serializing tuple structs is not supported").into())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(anyhow::anyhow!("Serializing tuple variants is not supported").into())
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.ser.writer.write_u8(FieldType::Compound as u8)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.ser.writer.write_u8(FieldType::Compound as u8)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(anyhow::anyhow!("Serializing struct variants is not supported").into())
    }
}

impl<'a, W, F> SerializeSeq for FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, _element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<'a, W, F> SerializeTuple for FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, _element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<'a, W, F> SerializeMap for FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_key<K>(&mut self, _key: &K) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_value<V>(&mut self, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<'a, W, F> SerializeStruct for FieldTypeSerializer<'a, W, F>
where
    W: BinaryWrite,
    F: VariantImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_field<V>(&mut self, _key: &'static str, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
