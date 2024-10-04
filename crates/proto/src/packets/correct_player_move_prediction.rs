use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 161)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct CorrectPlayerMovePredictionPacket {
    // TODO: Possibly turn this into an enum
    pub prediction_type: u8,
    #[endianness(le)]
    pub pos: Vec3<f32>,
    #[endianness(le)]
    pub pos_delta: Vec3<f32>,
    pub on_ground: bool,
    #[endianness(le)]
    pub tick: u64,
}
