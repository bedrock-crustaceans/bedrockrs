use crate::version::v662::enums::GameType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 105)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetDefaultGameTypePacket {
    pub default_game_type: GameType,
}