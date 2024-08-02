use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusUpdatedPacket {
    pub chunk_radius: VAR<u32>,
}
