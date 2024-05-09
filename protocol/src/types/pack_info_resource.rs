use bedrock_core::types::u64le;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct ResourcePackInfoType {
    id: String,
    version: String,
    size: u64le,
    content_key: String,
    sub_pack_name: String,
    content_identify: String,
    has_scripts: bool,
    ray_tracing_capable: bool,
}
