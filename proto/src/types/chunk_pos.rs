use bedrockrs_core::int::{VAR, LE};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPos {
    pub x: VAR<i32>,
    pub z: VAR<i32>
}
