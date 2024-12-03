use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::WebSocketPacketData;

#[gamepacket(id = 95)]
#[derive(ProtoCodec)]
pub struct AutomationClientConnectPacket {
    pub web_socket_data: WebSocketPacketData,
}