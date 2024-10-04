use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::dimension::Dimension;

#[gamepacket(id = 61)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChangeDimensionPacket {
    pub dimension: Dimension,
    #[endianness(le)]
    pub pos: Vec3<f32>,
    pub respawn: bool,
    #[endianness(le)]
    pub loading_screen: Option<u32>,
}
