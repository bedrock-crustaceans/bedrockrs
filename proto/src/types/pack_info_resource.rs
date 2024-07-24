use bedrockrs_core::LE;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePackInfoType {
    id: String,
    version: String,
    size: LE<u64>,
    content_key: String,
    sub_pack_name: String,
    content_identify: String,
    has_scripts: bool,
    ray_tracing_capable: bool,
}
