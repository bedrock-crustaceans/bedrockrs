use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

use crate::types::command_origin_data::CommandOriginData;

#[gamepacket(id = 77)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct CommandRequestPacket {
    command: String,
    command_origin: CommandOriginData,
    is_internal_source: bool,
    version: VAR<i32>,
}
