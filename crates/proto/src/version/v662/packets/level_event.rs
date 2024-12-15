use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::LevelEvent;
use crate::version::v662::types::Vec3;

#[gamepacket(id = 25)]
#[derive(ProtoCodec)]
pub struct LevelEventPacket {
    pub event_id: LevelEvent,
    pub position: Vec3,
    #[endianness(var)]
    pub data: i32,
}