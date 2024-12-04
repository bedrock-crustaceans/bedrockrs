use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::Puv;
use crate::version::v662::types::Vec3;

#[gamepacket(id = 120)]
#[derive(ProtoCodec)]
pub struct LevelSoundEventPacketV2 {
    pub event_id: Puv::Legacy::LevelSoundEvent, // TODO: listed as byte, enum is unsigned varint
    pub position: Vec3,
    #[endianness(var)]
    pub data: i32,
    pub actor_identifier: String,
    pub baby_mod: bool,
    pub global: bool,
}

// TODO: custom proto impl because of that stupid enum