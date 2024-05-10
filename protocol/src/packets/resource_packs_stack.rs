use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

use crate::types::base_game_version::BaseGameVersion;
use crate::types::experiments::Experiments;
use crate::types::resource_packs_stack_pack::ResourcePacksStackPack;

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct ResourcePacksStackPacket {
    pub texture_pack_required: bool,
    pub addons: Vec<ResourcePacksStackPack>,
    pub texture_packs: Vec<ResourcePacksStackPack>,
    pub base_game_version: BaseGameVersion,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}
