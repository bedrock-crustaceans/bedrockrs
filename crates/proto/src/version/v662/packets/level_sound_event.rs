use crate::version::v662::enums::Puv;
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 123)]
#[derive(ProtoCodec)]
pub struct LevelSoundEventPacket {
    pub event_id: Puv::Legacy::LevelSoundEvent,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub data: i32,
    pub actor_identifier: String,
    pub is_baby_mob: bool,
    pub is_global: bool,
}