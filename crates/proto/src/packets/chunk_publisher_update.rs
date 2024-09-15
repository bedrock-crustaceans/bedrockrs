use crate::types::block_pos::BlockPos;
use crate::types::chunk_pos::ChunkPos;
use bedrockrs_core::int::LE;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 121)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPublisherUpdatePacket {
    pub position: BlockPos,
    pub radius: LE<u32>,
    #[len_repr(VAR::<u32>)]
    pub saved_chunks: Vec<ChunkPos>,
}
