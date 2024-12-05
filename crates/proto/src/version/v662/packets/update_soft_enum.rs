use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::SoftEnumUpdateType;

#[gamepacket(id = 114)]
#[derive(ProtoCodec)]
pub struct UpdateSoftEnumPacket {
    pub enum_name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub values: Vec<String>,
    pub update_type: SoftEnumUpdateType,
}