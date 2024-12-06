use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{ActorType, Puv};
use crate::version::v662::types::Vec3;

#[gamepacket(id = 24)]
#[derive(ProtoCodec)]
pub struct LevelSoundEventPacketV1 {
    pub event_id: Puv::Legacy::LevelSoundEvent, // TODO: packet uses byte, but enum is unsigned varint
    pub position: Vec3,
    #[endianness(var)]
    pub data: i32,
    pub actor_type: ActorType,
    pub baby_mob: bool,
    pub global: bool,
}