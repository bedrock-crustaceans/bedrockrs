use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::SubChunkPacket::SubChunkPosOffset;
use crate::version::v662::types::SubChunkPos;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum HeightMapDataType {
    NoData = 0,
    HasData = 1,
    AllTooHigh = 2,
    AllTooLow = 3,
}

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum SubChunkRequestResult {
    Undefined = 0,
    Success = 1,
    LevelChunkDoesntExist = 2,
    WrongDimension = 3,
    PlayerDoesntExist = 4,
    IndexOutOfBounds = 5,
    SuccessAllAir = 6,
}

#[derive(ProtoCodec)]
struct SubChunkDataEntry {
    pub sub_chunk_pos_offset: SubChunkPosOffset,
    pub sub_chunk_request_result: SubChunkRequestResult,
    pub serialized_sub_chunk: Option<String>, // TODO: custom proto, if sub_chunk_request_result == SuccessAllAir, or cache_enabled
    pub height_map_data_type: HeightMapDataType,
    pub sub_chunk_height_map: Option<Vec<Vec<i8>>>, // TODO: custom proto, if height_map_data_type == HasData (vec sizes are i8)
    pub blob_id: Option<u64>, // TODO: custom proto, if cache_enabled
}

#[gamepacket(id = 174)]
#[derive(ProtoCodec)]
pub struct SubChunkPacket {
    pub cache_enabled: bool,
    #[endianness(var)]
    pub dimension_type: i32,
    pub center_pos: SubChunkPos,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub sub_chunk_data: Vec<SubChunkDataEntry>,
}