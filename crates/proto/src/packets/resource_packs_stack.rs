use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

use crate::types::base_game_version::BaseGameVersion;
use crate::types::experiments::Experiments;
use crate::types::resource_packs_stack_pack::ResourcePacksStackPack;

#[gamepacket(id = 7)]
#[derive(Debug, Clone, ProtoCodec)]
pub struct ResourcePacksStackPacket {
    pub texture_pack_required: bool,
    #[len_repr(VAR::<i32>)]
    pub addons: Vec<ResourcePacksStackPack>,
    #[len_repr(VAR::<i32>)]
    pub texture_packs: Vec<ResourcePacksStackPack>,
    pub base_game_version: BaseGameVersion,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}
