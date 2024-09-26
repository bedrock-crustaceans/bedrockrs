use bedrockrs_core::int::VAR;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 69)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusRequestPacket {
    pub chunk_radius: VAR<i32>,
    pub chunk_radius_max: u8,
}
