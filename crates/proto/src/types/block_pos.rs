use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct BlockPos {
    pub x: VAR<i32>,
    pub y: VAR<u32>,
    pub z: VAR<i32>,
}
