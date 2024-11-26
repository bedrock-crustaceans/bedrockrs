use bedrockrs_core::Vec3;
use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{BlockPos, CompoundTag};

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum DataItemType {
    Byte(i8) = 0,
    #[endianness(le)]
    Short(i16) = 1,
    #[endianness(var)]
    Int(i32) = 2,
    #[endianness(le)]
    Float(f32) = 3,
    String(String) = 4,
    CompoundTag(CompoundTag) = 5,
    Pos(BlockPos) = 6,
    #[endianness(var)]
    Int64(i64) = 7,
    Vec3(Vec3<f32>) = 8,
}