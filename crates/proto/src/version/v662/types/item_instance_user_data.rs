use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::CompoundTag;

#[derive(ProtoCodec)]
pub struct ItemInstanceUserData {
    #[endianness(le)]
    pub serialization_marker: i16,
    pub serialization_version: i8,
    pub tags: CompoundTag,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub can_place_on: Vec<String>,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub can_break: Vec<String>,
}