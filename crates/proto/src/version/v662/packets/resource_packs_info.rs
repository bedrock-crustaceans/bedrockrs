use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
struct BehaviourPackEntry {
    pub id: String,
    pub version: String,
    #[endianness(le)]
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
}

#[derive(ProtoCodec)]
struct ResourcePackEntry {
    pub id: String,
    pub version: String,
    #[endianness(le)]
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub is_ray_tracing_capable: bool,
}

#[derive(ProtoCodec)]
struct CDNUrl {
    pub first: String,
    pub second: String,
}

#[gamepacket(id = 6)]
#[derive(ProtoCodec)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub behaviour_packs: Vec<BehaviourPackEntry>,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub resource_packs: Vec<ResourcePackEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub cdn_urls: Vec<CDNUrl>,
}