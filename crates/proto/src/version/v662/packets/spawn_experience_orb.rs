use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 66)]
#[derive(ProtoCodec)]
pub struct SpawnExperienceOrbPacket {
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub xp_value: i32,
}