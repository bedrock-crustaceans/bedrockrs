use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct BlockPos {
    pub x: VAR<i32>,
    pub y: VAR<u32>,
    pub z: VAR<i32>,
}

impl BlockPos {
    pub fn new(x: i32, y: u32, z: i32) -> BlockPos {
        Self {
            x: VAR::new(x),
            y: VAR::new(y),
            z: VAR::new(z),
        }
    }
}
