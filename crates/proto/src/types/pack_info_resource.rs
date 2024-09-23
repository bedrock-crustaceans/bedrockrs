use bedrockrs_core::int::LE;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePackInfoType {
    pub id: String,
    pub version: String,
    pub size: LE<u64>,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identify: String,
    pub has_scripts: bool,
    pub ray_tracing_capable: bool,
}
