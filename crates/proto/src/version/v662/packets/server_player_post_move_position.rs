use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec3;

#[gamepacket(id = 16)]
#[derive(ProtoCodec)]
pub struct ServerPlayerPostMovePositionPacket {
    pub pos: Vec3,
}