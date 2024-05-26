use bedrock_core::u64le;
use proto_derive::ProtoCodec;

#[derive(Debug, ProtoCodec)]
pub struct BehaviorPackInfoType {
    id: String,
    version: String,
    size: u64le,
    content_key: String,
    sub_pack_name: String,
    content_identify: String,
    has_scripts: bool,
}
