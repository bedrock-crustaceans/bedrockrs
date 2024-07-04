use bedrock_core::VAR;
use proto_derive::ProtoCodec;
use crate::types::player_movement_mode::PlayerMovementMode;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerMovementSettings {
    authority_mode: PlayerMovementMode,
    rewind_history_size: VAR<i32>,
    server_authoritative_block_breaking: bool
}
