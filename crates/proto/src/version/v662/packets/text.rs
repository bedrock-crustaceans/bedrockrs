use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::TextPacketType;

#[gamepacket(id = 9)]
#[derive(ProtoCodec)]
pub struct TextPacket {
    pub message_type: TextPacketType,
    pub localize: bool,
    pub sender_xuid: String,
    pub platform_id: String,
}

// TODO: custom proto impl, because of enum variant serialization order
