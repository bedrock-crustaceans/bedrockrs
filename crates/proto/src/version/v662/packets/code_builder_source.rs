use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::CodeBuilderStorageQueryOptions;

#[gamepacket(id = 178)]
#[derive(ProtoCodec)]
pub struct CodeBuilderSourcePacket {
    pub operation: CodeBuilderStorageQueryOptions::Operation,
    pub category: CodeBuilderStorageQueryOptions::Category,
    pub value: String,
}