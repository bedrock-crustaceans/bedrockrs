use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 69)]
#[derive(ProtoCodec)]
pub struct RequestChunkRadiusPacket {
    #[endianness(var)]
    pub chunk_radius: i32,
    pub max_chunk_radius: i8,
}