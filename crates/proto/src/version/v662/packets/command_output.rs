use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::CommandOutputType;
use crate::version::v662::types::CommandOriginData;

#[derive(ProtoCodec)]
struct OutputMessagesEntry {
    pub message_id: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub parameters: Vec<String>,
}

#[gamepacket(id = 79)]
#[derive(ProtoCodec)]
pub struct CommandOutputPacket {
    pub origin_data: CommandOriginData,
    pub output_type: CommandOutputType,
    pub success_count: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub output_messages: Vec<OutputMessagesEntry>,
}

// TODO: custom proto impl, enum variant