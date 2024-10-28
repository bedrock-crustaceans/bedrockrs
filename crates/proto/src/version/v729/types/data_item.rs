use crate::version::v729::types::block_pos::BlockPos;
use bedrockrs_core::Vec3;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct DataItem {
    #[endianness(var)]
    pub id: u32,
    pub value: DataItemValue,
}

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum DataItemValue {
    ValByte(u8) = 0,
    ValShort(#[endianness(le)] i16) = 1,
    ValInt(#[endianness(var)] i32) = 2,
    ValFloat(#[endianness(le)] f32) = 3,
    ValString(String) = 4,
    ValCompoundTag(#[nbt] nbtx::Value) = 5,
    ValBlockPos(BlockPos) = 6,
    ValInt64(#[endianness(var)] i64) = 7,
    ValVec3(#[endianness(le)] Vec3<f32>) = 8,
}
