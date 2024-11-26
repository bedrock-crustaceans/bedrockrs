use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{ActorRuntimeID, Vec3};

#[derive(ProtoCodec)]
pub struct MoveActorAbsoluteData {
    pub actor_runtime_id: ActorRuntimeID,
    pub header: i8,
    pub position: Vec3,
    pub rotation_x: i8,
    pub rotation_y: i8,
    pub rotation_y_head: i8,
}