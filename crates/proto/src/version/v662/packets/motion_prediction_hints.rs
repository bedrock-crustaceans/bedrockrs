use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[gamepacket(id = 157)]
#[derive(ProtoCodec)]
pub struct MotionPredictionHintsPacket {
    pub runtime_id: ActorRuntimeID,
    #[endianness(le)]
    pub motion: Vec3<f32>,
    pub on_ground: bool,
}