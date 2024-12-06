use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum Type {
    End = 0,
    Byte(i8) = 1,
    #[endianness(le)]
    Short(i16) = 2,
    #[endianness(var)]
    Int(i32) = 3,
    #[endianness(var)]
    Int64(i64) = 4,
    #[endianness(le)]
    Float(f32) = 5,
    #[endianness(le)]
    Double(f64) = 6,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    ByteArray(Vec<i8>) = 7,
    String(String) = 8,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    List(Vec<CompoundTag>) = 9,
    Compound(CompoundTag) = 10,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    #[endianness(var)]
    IntArray(Vec<i32>) = 11,
}

#[derive(ProtoCodec)]
pub struct CompoundTag {
    pub tag_type: Type,
    pub tag_name: String,
}

// TODO: custom proto impl, because of enum variant serialization order.