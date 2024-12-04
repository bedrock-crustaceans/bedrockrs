use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::StructureTemplateResponseType;
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 133)]
#[derive(ProtoCodec)]
pub struct StructureDataResponsePacket {
    pub structure_name: String,
    pub structure_nbt: Option<CompoundTag>, 
    pub response_type: StructureTemplateResponseType,
}

// TODO: make sure this actually works