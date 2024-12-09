use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::StructureTemplateResponseType;

#[gamepacket(id = 133)]
#[derive(ProtoCodec)]
pub struct StructureDataResponsePacket {
    pub structure_name: String,
    #[nbt]
    pub structure_nbt: Option<nbtx::Value>, // TODO: NBT Structure
    pub response_type: StructureTemplateResponseType,
}

// VERIFY: If this actually works