use crate::version::v729::types::block_pos::BlockPos;
use crate::version::v729::types::chunk_pos::ChunkPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 121)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPublisherUpdatePacket {
    pub position: BlockPos,
    #[endianness(var)]
    pub radius: u32,
    #[vec_repr(u32)]
    // TODO: Figure out if it is a var or le
    #[vec_endianness(var)]
    pub saved_chunks: Vec<ChunkPos>,
}
