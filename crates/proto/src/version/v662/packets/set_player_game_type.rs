use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::GameType;

#[gamepacket(id = 62)]
#[derive(ProtoCodec)]
pub struct SetPlayerGameTypePacket {
    pub player_game_type: GameType,
}