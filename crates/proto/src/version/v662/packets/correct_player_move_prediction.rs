use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PredictionType;
use crate::version::v662::types::Vec3;

#[gamepacket(id = 161)]
#[derive(ProtoCodec)]
pub struct CorrectPlayerMovePredictionPacket {
    pub position: Vec3,
    pub velocity: Vec3,
    pub on_ground: bool,
    #[endianness(var)]
    pub tick: u64,
    pub prediction_type: PredictionType,
}