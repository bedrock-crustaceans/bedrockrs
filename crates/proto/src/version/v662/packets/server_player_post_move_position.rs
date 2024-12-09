use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 16)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ServerPlayerPostMovePositionPacket {
    #[endianness(le)]
    pub pos: Vec3<f32>,
}