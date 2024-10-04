use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 70)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkRadiusUpdatedPacket {
    #[endianness(var)]
    pub chunk_radius: i32,
}
