use uuid::Uuid;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Experiments;

#[gamepacket(id = 7)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePacksStackPacket {
    pub resource_pack_required: bool,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub behavior_packs: Vec<StackResourcePack>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub resource_packs: Vec<StackResourcePack>,
    pub base_game_version: String,
    pub experiments: Experiments
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct StackResourcePack {
    #[str]
    pub uuid: Uuid,
    pub version: String,
    pub sub_pack: String
}
