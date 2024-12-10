use crate::version::v662::enums::{StructureBlockType, StructureRedstoneSaveMode};
use crate::version::v662::types::StructureSettings;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct StructureEditorData {
    pub structure_name: String,
    pub data_field: String,
    pub include_players: bool,
    pub show_bounding_box: bool,
    pub structure_block_type: StructureBlockType,
    pub structure_settings: StructureSettings,
    pub redstone_save_mode: StructureRedstoneSaveMode
}
