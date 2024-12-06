use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CommandOriginData;

#[gamepacket(id = 77)]
#[derive(ProtoCodec)]
pub struct CommandRequestPacket {
    pub command: String,
    pub command_origin: CommandOriginData,
    pub is_internal_source: bool,
    #[endianness(var)]
    pub version: i32,
}