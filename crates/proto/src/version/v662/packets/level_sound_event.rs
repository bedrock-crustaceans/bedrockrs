use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::Puv;
use crate::version::v662::types::Vec3;

#[gamepacket(id = 123)]
#[derive(ProtoCodec)]
pub struct LevelSoundEventPacket {
    pub event_id: Puv::Legacy::LevelSoundEvent,
    pub position: Vec3,
    #[endianness(var)]
    pub data: i32,
    pub actor_identifier: String,
    pub is_baby_mob: bool,
    pub is_global: bool,
}