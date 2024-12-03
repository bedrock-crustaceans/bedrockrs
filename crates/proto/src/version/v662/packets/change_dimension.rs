use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec3;

#[gamepacket(id = 61)]
#[derive(ProtoCodec)]
pub struct ChangeDimensionPacket {
    #[endianness(var)]
    pub dimension_id: i32,
    pub position: Vec3,
    pub respawn: bool,
}