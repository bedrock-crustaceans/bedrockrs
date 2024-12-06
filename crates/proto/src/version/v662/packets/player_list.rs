use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PlayerListPacketType;

#[gamepacket(id = 63)]
#[derive(ProtoCodec)]
pub struct PlayerListPacket {
    pub action: PlayerListPacketType,
}