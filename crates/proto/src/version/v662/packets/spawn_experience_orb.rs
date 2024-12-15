use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec3;

#[gamepacket(id = 66)]
#[derive(ProtoCodec)]
pub struct SpawnExperienceOrbPacket {
    pub position: Vec3,
    #[endianness(var)]
    pub xp_value: i32,
}