use bedrock_core::VAR;
use proto_derive::ProtoCodec;

use crate::types::base_game_version::BaseGameVersion;
use crate::types::experiments::Experiments;
use crate::types::resource_packs_stack_pack::ResourcePacksStackPack;

#[derive(Debug, Clone, ProtoCodec)]
pub struct ResourcePacksStackPacket {
    pub texture_pack_required: bool,
    #[len_type(VAR::< i32 >)]
    pub addons: Vec<ResourcePacksStackPack>,
    #[len_type(VAR::< i32 >)]
    pub texture_packs: Vec<ResourcePacksStackPack>,
    pub base_game_version: BaseGameVersion,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}
