use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// General NBT value type that can represent any value.
///
/// In case the structure of some piece of NBT data is not known, this
/// type can be used to deserialise it.
#[derive(Debug, Clone)]
pub enum Value {
    /// A signed byte.
    Byte(i8),
    /// A signed short.
    Short(i16),
    /// A signed int.
    Int(i32),
    /// A signed long.
    Long(i64),
    /// A signed float
    Float(f32),
    /// A signed double
    Double(f64),
    /// A byte array.
    ///
    /// This type is not used when deserialising due to issues with `serde`.
    /// In case you are defining your own types, you can use [`serde_bytes`](https://crates.io/crates/serde_bytes)
    /// to make use of the byte array type.
    ByteArray(Vec<u8>),
    /// A UTF-8 string.
    String(String),
    /// List of an arbitrary NBT value.
    List(Vec<Value>),
    /// Key-value map.
    Compound(HashMap<String, Value>),
    /// An array of integers.
    IntArray(Vec<i32>),
    /// An array of longs.
    LongArray(Vec<i64>),
}

macro_rules! impl_access_fns {
    ($($tag: ident = $ty: ty),+) => {
        $(paste::paste! {
            #[inline]
            #[doc = concat!(
                "Returns the inner value if the tag is of a, ", stringify!($tag), ", otherwise returns self.
                This method is the same as [`as_", stringify!([<$tag:snake>]), "`](Self::as_", stringify!([<$tag:snake>]), ") but instead takes ownership of the value."
            )]
            pub fn [<into_ $tag:snake>](self) -> Result<$ty, Self> {
                match self {
                    Self::$tag(val) => Ok(val),
                    _ => Err(self)
                }
            }

            #[inline]
            #[doc = concat!(
                "Returns a reference to the inner value of the tag is the requested type is present.
                Use [`into_", stringify!([<$tag:snake>]), "`](Self::into_", stringify!([<$tag:snake>]), ")."
            )]
            pub fn [<as_ $tag:snake>](&self) -> Option<&$ty> {
                match self {
                    Self::$tag(val) => Some(val),
                    _ => None
                }
            }

            #[inline]
            #[doc = concat!(
                "Returns whether the inner value is of type `", stringify!($tag), "`."
            )]
            pub fn [<is_ $tag:snake>](&self) -> bool {
                matches!(self, Self::$tag(_))
            }
        })+
    }
}

impl Value {
    impl_access_fns!(
        Byte = i8,
        Short = i16,
        Int = i32,
        Long = i64,
        Float = f32,
        Double = f64,
        String = String,
        List = Vec<Self>,
        Compound = HashMap<String, Self>,
        ByteArray = Vec<u8>,
        IntArray = Vec<i32>,
        LongArray = Vec<i64>
    );
}

impl PartialEq<Value> for Value {
    #[inline]
    fn eq(&self, rhs: &Value) -> bool {
        match self {
            Value::Byte(lhs) => rhs.as_byte().map_or(false, |rhs| lhs == rhs),
            Value::Short(lhs) => rhs.as_short().map_or(false, |rhs| lhs == rhs),
            Value::Int(lhs) => rhs.as_int().map_or(false, |rhs| lhs == rhs),
            Value::Long(lhs) => rhs.as_long().map_or(false, |rhs| lhs == rhs),
            Value::Float(lhs) => rhs.as_float().map_or(false, |rhs| lhs == rhs),
            Value::Double(lhs) => rhs.as_double().map_or(false, |rhs| lhs == rhs),
            Value::ByteArray(lhs) => rhs
                .as_byte_array()
                .map_or(false, |rhs| lhs.as_slice() == rhs),
            Value::String(lhs) => rhs.as_string().map_or(false, |rhs| lhs == rhs),
            Value::List(lhs) => rhs.as_list().map_or(false, |rhs| lhs == rhs),
            Value::Compound(lhs) => rhs.as_compound().map_or(false, |rhs| lhs == rhs),
            Value::IntArray(lhs) => rhs.as_int_array().map_or(false, |rhs| lhs == rhs),
            Value::LongArray(lhs) => rhs.as_long_array().map_or(false, |rhs| lhs == rhs),
        }
    }
}

impl PartialEq<i8> for Value {
    #[inline]
    fn eq(&self, rhs: &i8) -> bool {
        self.as_byte().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i8> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &i8) -> bool {
        self.as_byte().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i8> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &i8) -> bool {
        self.as_byte().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<i16> for Value {
    #[inline]
    fn eq(&self, rhs: &i16) -> bool {
        self.as_short().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i16> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &i16) -> bool {
        self.as_short().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i16> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &i16) -> bool {
        self.as_short().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<i32> for Value {
    #[inline]
    fn eq(&self, rhs: &i32) -> bool {
        self.as_int().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i32> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &i32) -> bool {
        self.as_int().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i32> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &i32) -> bool {
        self.as_int().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<i64> for Value {
    #[inline]
    fn eq(&self, rhs: &i64) -> bool {
        self.as_long().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i64> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &i64) -> bool {
        self.as_long().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<i64> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &i64) -> bool {
        self.as_long().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<f32> for Value {
    #[inline]
    fn eq(&self, rhs: &f32) -> bool {
        self.as_float().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<f32> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &f32) -> bool {
        self.as_float().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<f32> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &f32) -> bool {
        self.as_float().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<f64> for Value {
    #[inline]
    fn eq(&self, rhs: &f64) -> bool {
        self.as_double().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<f64> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &f64) -> bool {
        self.as_double().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<f64> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &f64) -> bool {
        self.as_double().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[u8]> for Value {
    #[inline]
    fn eq(&self, rhs: &&[u8]) -> bool {
        self.as_byte_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[u8]> for &Value {
    #[inline]
    fn eq(&self, rhs: &&[u8]) -> bool {
        self.as_byte_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[u8]> for &mut Value {
    #[inline]
    fn eq(&self, rhs: &&[u8]) -> bool {
        self.as_byte_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&str> for Value {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self.as_string().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&str> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self.as_string().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&str> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self.as_string().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[Value]> for Value {
    #[inline]
    fn eq(&self, rhs: &&[Value]) -> bool {
        self.as_list().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[Value]> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &&[Value]) -> bool {
        self.as_list().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[Value]> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &&[Value]) -> bool {
        self.as_list().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<HashMap<String, Value>> for Value {
    #[inline]
    fn eq(&self, rhs: &HashMap<String, Value>) -> bool {
        self.as_compound().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<HashMap<String, Value>> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &HashMap<String, Value>) -> bool {
        self.as_compound().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<HashMap<String, Value>> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &HashMap<String, Value>) -> bool {
        self.as_compound().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[i32]> for Value {
    #[inline]
    fn eq(&self, rhs: &&[i32]) -> bool {
        self.as_int_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[i32]> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &&[i32]) -> bool {
        self.as_int_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[i32]> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &&[i32]) -> bool {
        self.as_int_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl PartialEq<&[i64]> for Value {
    #[inline]
    fn eq(&self, rhs: &&[i64]) -> bool {
        self.as_long_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[i64]> for &'a Value {
    #[inline]
    fn eq(&self, rhs: &&[i64]) -> bool {
        self.as_long_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl<'a> PartialEq<&[i64]> for &'a mut Value {
    #[inline]
    fn eq(&self, rhs: &&[i64]) -> bool {
        self.as_long_array().map_or(false, |lhs| lhs == rhs)
    }
}

impl Hash for Value {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            Value::Byte(v) => state.write_i8(*v),
            Value::Short(v) => state.write_i16(*v),
            Value::Int(v) => state.write_i32(*v),
            Value::Long(v) => state.write_i64(*v),
            Value::String(v) => state.write(v.as_bytes()),
            Value::Float(v) => {
                // f32 does not implement Hash so simply hash the byte repr.
                state.write(&v.to_le_bytes());
            }
            Value::Double(v) => {
                // f64 does not implement Hash so simply hash the byte repr.
                state.write(&v.to_le_bytes());
            }
            Value::Compound(map) => {
                for (k, v) in map {
                    state.write(k.as_bytes());
                    v.hash(state);
                }
            }
            Value::List(v) => Self::hash_slice(v, state),
            Value::ByteArray(v) => u8::hash_slice(v, state),
            Value::IntArray(v) => i32::hash_slice(v, state),
            Value::LongArray(v) => i64::hash_slice(v, state),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

#[inline]
fn serialize_seq<T, S>(ser: S, seq: &[T]) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut seq_ser = ser.serialize_seq(Some(seq.len()))?;
    for element in seq {
        seq_ser.serialize_element(element)?;
    }
    seq_ser.end()
}

impl Serialize for Value {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Byte(byte) => ser.serialize_i8(*byte),
            Value::Short(short) => ser.serialize_i16(*short),
            Value::Int(int) => ser.serialize_i32(*int),
            Value::Long(long) => ser.serialize_i64(*long),
            Value::Float(float) => ser.serialize_f32(*float),
            Value::Double(double) => ser.serialize_f64(*double),
            Value::ByteArray(array) => ser.serialize_bytes(array),
            Value::String(string) => ser.serialize_str(string),
            Value::List(seq) => serialize_seq(ser, seq),
            Value::Compound(map) => {
                let mut map_ser = ser.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(k, v)?;
                }
                map_ser.end()
            }
            Value::IntArray(seq) => serialize_seq(ser, seq),
            Value::LongArray(seq) => serialize_seq(ser, seq),
        }
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid NBT value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Byte(v as i8))
    }

    #[inline]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Byte(v))
    }

    #[inline]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Short(v))
    }

    #[inline]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Int(v))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Long(v))
    }

    #[inline]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Float(v))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Double(v))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(v.to_owned()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(v))
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::ByteArray(v))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out = Vec::new();
        if let Some(hint) = seq.size_hint() {
            out.reserve(hint);
        }

        while let Some(element) = seq.next_element()? {
            out.push(element);
        }

        Ok(Value::List(out))
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut out: HashMap<String, Value> = HashMap::new();
        if let Some(hint) = map.size_hint() {
            out.reserve(hint);
        }

        while let Some((key, value)) = map.next_entry()? {
            out.insert(key, value);
        }

        Ok(Value::Compound(out))
    }
}
