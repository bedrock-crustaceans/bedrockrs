use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct BlockPos {
    #[endianness(var)]
    pub x: i32,
    #[endianness(var)]
    pub y: i32,
    #[endianness(var)]
    pub z: i32,
}