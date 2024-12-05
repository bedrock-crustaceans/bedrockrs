use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{NetworkBlockPosition, StructureEditorData};

#[gamepacket(id = 90)]
#[derive(ProtoCodec)]
pub struct StructureBlockUpdatePacket {
    pub block_position: NetworkBlockPosition,
    pub structure_data: StructureEditorData,
    pub trigger: bool,
    pub is_waterlogged: bool,
}