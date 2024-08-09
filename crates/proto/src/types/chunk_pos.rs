use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPos {
    pub x: VAR<i32>,
    pub z: VAR<i32>,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkPos {
            x: VAR::<i32>::new(x),
            z: VAR::<i32>::new(z),
        }
    }
}
