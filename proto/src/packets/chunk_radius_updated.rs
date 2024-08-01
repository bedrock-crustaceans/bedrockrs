use bedrockrs_core::int::{VAR, LE};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusUpdatedPacket {
    pub chunk_radius: VAR<u32>
}
