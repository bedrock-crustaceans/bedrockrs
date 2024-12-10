use crate::version::v662::types::SubChunkPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::SubChunkPosOffset;

#[gamepacket(id = 175)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SubChunkRequestPacket {
    #[endianness(var)]
    pub dimension_type: i32,
    pub center_pos: SubChunkPos,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub sub_chunk_pos_offsets: Vec<SubChunkPosOffset>
}