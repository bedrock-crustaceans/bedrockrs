use crate::version::v662::enums::GameType;
use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 151)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct UpdatePlayerGameTypePacket {
    pub player_game_type: GameType,
    pub target_player: ActorUniqueID,
}