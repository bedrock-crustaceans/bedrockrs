use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::GameType;
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 151)]
#[derive(ProtoCodec)]
pub struct UpdatePlayerGameTypePacket {
    pub player_game_type: GameType,
    pub target_player: ActorUniqueID,
}