use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::GameType;

#[gamepacket(id = 105)]
#[derive(ProtoCodec)]
pub struct SetDefaultGameTypePacket {
    pub default_game_type: GameType,
}