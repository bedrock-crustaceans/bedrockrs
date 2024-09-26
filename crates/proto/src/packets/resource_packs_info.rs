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
    #[len_repr(LE::<u16>)]
    pub behavior_packs: Vec<BehaviorPackInfoType>,
    #[len_repr(LE::<u16>)]
    pub resource_packs: Vec<ResourcePackInfoType>,
    #[len_repr(VAR::<u32>)]
    pub cdn_urls: Vec<PackURL>,
}
