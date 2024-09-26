use bedrockrs_core::{int::LE, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 18)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerPlayerPostMovePositionPacket {
    pub pos: Vec3<LE<f32>>,
}
