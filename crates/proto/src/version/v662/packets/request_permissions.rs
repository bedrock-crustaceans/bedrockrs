use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PlayerPermissionLevel;

#[gamepacket(id = 185)]
#[derive(ProtoCodec)]
pub struct RequestPermissionsPacket {
    #[endianness(le)]
    pub target_player_raw_id: i64,
    pub player_permission_level: PlayerPermissionLevel,
    #[endianness(le)]
    pub custom_permission_flags: u16,
}