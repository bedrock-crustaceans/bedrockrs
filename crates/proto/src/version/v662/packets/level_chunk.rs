use crate::version::v662::types::ChunkPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
struct CacheBlobEntry {
    #[endianness(var)]
    blob: u64
}

#[gamepacket(id = 58)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct LevelChunkPacket {
    pub chunk_position: ChunkPos,
    #[endianness(var)]
    pub dimension_id: i32,
    // TODO: sub-chunk count stuff
    pub cache_enabled: bool,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub cache_blobs: Vec<CacheBlobEntry>,
    pub serialized_chunk_data: String,
}

// TODO: this whole thing is terrible