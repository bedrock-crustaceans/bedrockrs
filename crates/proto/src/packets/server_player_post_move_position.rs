use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 18)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerPlayerPostMovePositionPacket {
    #[endianness(le)]
    pub pos: Vec3<f32>,
}
