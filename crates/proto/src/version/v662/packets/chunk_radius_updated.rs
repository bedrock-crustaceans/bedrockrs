use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 70)]
#[derive(ProtoCodec)]
pub struct ChunkRadiusUpdatedPacket {
    #[endianness(var)]
    pub chunk_radius: i32,
}