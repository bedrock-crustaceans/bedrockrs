use crate::version::v729::types::text_message_data::TextMessageData;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use xuid::Xuid;

#[gamepacket(id = 9)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct TextMessagePacket {
    pub message_type: TextMessageData,
    pub sender_xuid: Xuid,
    pub platform_id: String,
    pub filtered_message: String,
}
