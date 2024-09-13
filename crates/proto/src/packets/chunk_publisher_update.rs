use bedrockrs_core::int::LE;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use crate::types::chunk_pos::ChunkPos;
use crate::types::network_block_pos::NetworkBlockPos;

#[gamepacket(id = 121)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChunkPublisherUpdatePacket {
    position: NetworkBlockPos,
    radius: LE<u32>,
    #[len_repr(VAR::<u32>)]
    saved_chunks: Vec<ChunkPos>,
}