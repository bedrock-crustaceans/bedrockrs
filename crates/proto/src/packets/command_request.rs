use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::types::command_origin_data::CommandOriginData;

#[gamepacket(id = 77)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct CommandRequestPacket {
    pub command: String,
    pub command_origin: CommandOriginData,
    pub is_internal_source: bool,
    #[endianness(var)]
    pub version: i32,
}
