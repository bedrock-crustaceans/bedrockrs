use bedrockrs_core::int::LE;
use bedrockrs_core::int::VAR;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::ProtoCodec;

use crate::types::pack_info_behavior::BehaviorPackInfoType;
use crate::types::pack_info_resource::ResourcePackInfoType;
use crate::types::pack_url::PackURL;

#[gamepacket(id = 6)]
#[derive(Debug, Clone, ProtoCodec)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub behavior_packs: Vec<BehaviorPackInfoType>,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub resource_packs: Vec<ResourcePackInfoType>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub cdn_urls: Vec<PackURL>,
}
