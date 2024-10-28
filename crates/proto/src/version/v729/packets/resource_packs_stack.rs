use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::version::v729::types::base_game_version::BaseGameVersion;
use crate::version::v729::types::experiments::Experiments;
use crate::version::v729::types::resource_packs_stack_pack::ResourcePacksStackPack;

#[gamepacket(id = 7)]
#[derive(Debug, Clone, ProtoCodec)]
pub struct ResourcePacksStackPacket {
    pub texture_pack_required: bool,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub addons: Vec<ResourcePacksStackPack>,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub texture_packs: Vec<ResourcePacksStackPack>,
    pub base_game_version: BaseGameVersion,
    pub experiments: Experiments,
    pub include_editor_packs: bool,
}
