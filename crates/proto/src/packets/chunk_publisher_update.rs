use crate::types::block_pos::BlockPos;
use crate::types::chunk_pos::ChunkPos;
use bedrockrs_core::int::LE;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 121)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPublisherUpdatePacket {
    position: BlockPos,
    radius: LE<u32>,
    #[len_repr(VAR::<u32>)]
    saved_chunks: Vec<ChunkPos>,
}
