use bedrockrs_core::int::LE;
use bedrockrs_core::Vec3;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::dimension::Dimension;

#[gamepacket(id = 61)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChangeDimensionPacket {
    dimension: Dimension,
    pos: Vec3<LE<f32>>,
    respawn: bool,
    loading_screen: Option<LE<u32>>,
}
