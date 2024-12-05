use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::SubChunkPacket;
use crate::version::v662::types::SubChunkPos;

#[gamepacket(id = 175)]
#[derive(ProtoCodec)]
pub struct SubChunkRequestPacket {
    #[endianness(var)]
    pub dimension_type: i32,
    pub center_pos: SubChunkPos,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub sub_chunk_pos_offsets: Vec<SubChunkPacket::SubChunkPosOffset>
}