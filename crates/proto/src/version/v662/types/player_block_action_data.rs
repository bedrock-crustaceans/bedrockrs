use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::PlayerActionType;

#[derive(ProtoCodec)]
pub struct PlayerBlockActionData {
    pub player_action_type: PlayerActionType
}