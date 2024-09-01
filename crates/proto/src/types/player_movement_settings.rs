use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::ProtoCodec;

use crate::types::player_movement_mode::PlayerMovementMode;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerMovementSettings {
    pub authority_mode: PlayerMovementMode,
    pub rewind_history_size: VAR<i32>,
    pub server_authoritative_block_breaking: bool,
}
