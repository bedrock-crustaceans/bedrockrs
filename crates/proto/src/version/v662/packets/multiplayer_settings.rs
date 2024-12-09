use crate::version::v662::enums::MultiplayerSettingsPacketType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 139)]
#[derive(ProtoCodec)]
pub struct MultiplayerSettingsPacket {
    pub multiplayer_settings_packet_type: MultiplayerSettingsPacketType,
}