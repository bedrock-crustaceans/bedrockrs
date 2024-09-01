use bedrockrs_core::{int::LE, Vec3};
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 161)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct CorrectPlayerMovePredictionPacket {
    pub prediction_type: u8,
    pub pos: Vec3<LE<f32>>,
    pub pos_delta: Vec3<LE<f32>>,
    pub on_ground: bool,
    pub tick: LE<u64>,
}
