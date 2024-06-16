use bedrock_core::LE;
use proto_derive::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct BehaviorPackInfoType {
    id: String,
    version: String,
    size: LE<u64>,
    content_key: String,
    sub_pack_name: String,
    content_identify: String,
    has_scripts: bool,
}
