use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 69)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusRequestPacket {
    #[endianness(var)]
    pub chunk_radius: i32,
    pub chunk_radius_max: u8,
}
