use bedrockrs_macros::ProtoCodec;

use crate::types::player_movement_mode::PlayerMovementMode;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerMovementSettings {
    pub authority_mode: PlayerMovementMode,
    #[endianness(var)]
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}
