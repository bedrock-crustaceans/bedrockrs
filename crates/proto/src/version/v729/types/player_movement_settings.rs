use bedrockrs_macros::ProtoCodec;

use crate::version::v729::types::player_movement_mode::PlayerMovementMode;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerMovementSettings {
    pub authority_mode: PlayerMovementMode,
    #[endianness(var)]
    pub rewind_history_size: i32,
    pub server_auth_block_breaking: bool,
}
