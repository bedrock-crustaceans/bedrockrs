use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, Vec3};

#[gamepacket(id = 157)]
#[derive(ProtoCodec)]
pub struct MotionPredictionHintsPacket {
    pub runtime_id: ActorRuntimeID,
    pub motion: Vec3,
    pub on_ground: bool,
}