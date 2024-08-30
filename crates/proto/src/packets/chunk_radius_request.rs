use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 69)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusRequestPacket {
    pub chunk_radius: VAR<u32>,
    pub chunk_radius_max: u8,
}
