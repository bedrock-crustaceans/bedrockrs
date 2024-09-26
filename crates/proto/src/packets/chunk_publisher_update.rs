use crate::types::block_pos::BlockPos;
use crate::types::chunk_pos::ChunkPos;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 121)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPublisherUpdatePacket {
    pub position: BlockPos,
    pub radius: VAR<u32>,
    #[len_repr(LE::<u32>)]
    pub saved_chunks: Vec<ChunkPos>,
}
