use bedrockrs_core::int::LE;
use bedrockrs_core::Vec3;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::dimension::Dimension;

#[gamepacket(id = 61)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChangeDimensionPacket {
    pub dimension: Dimension,
    pub pos: Vec3<LE<f32>>,
    pub respawn: bool,
    pub loading_screen: Option<LE<u32>>,
}
