use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ServerAuthMovementMode;

#[derive(ProtoCodec)]
pub struct SyncedPlayerMovementSettings {
    pub authority_mode: ServerAuthMovementMode,
    #[endianness(var)]
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}