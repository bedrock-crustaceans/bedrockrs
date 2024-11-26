use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::PlayerBlockActionData;

#[derive(ProtoCodec)]
pub struct PlayerBlockActions {
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub player_block_actions: Vec<PlayerBlockActionData>
}