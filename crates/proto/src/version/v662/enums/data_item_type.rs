use crate::version::v662::types::BlockPos;
use bedrockrs_core::Vec3;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum DataItemType {
    Byte(i8) = 0,
    Short(#[endianness(le)] i16) = 1,
    Int(#[endianness(var)] i32) = 2,
    Float(#[endianness(le)] f32) = 3,
    String(String) = 4,
    NBT(#[nbt] nbtx::Value) = 5,
    Pos(BlockPos) = 6,
    Int64(#[endianness(var)] i64) = 7,
    Vec3(#[endianness(le)] Vec3<f32>) = 8,
}