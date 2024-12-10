use crate::version::v662::enums::GameType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 62)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetPlayerGameTypePacket {
    pub player_game_type: GameType,
}