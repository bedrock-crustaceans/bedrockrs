use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec2;

#[gamepacket(id = 57)]
#[derive(ProtoCodec)]
pub struct PlayerInputPacket {
    pub move_vector: Vec2,
    pub jumping: bool,
    pub sneaking: bool,
}