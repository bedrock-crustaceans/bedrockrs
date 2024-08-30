use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 70)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusUpdatedPacket {
    pub chunk_radius: VAR<u32>,
}
