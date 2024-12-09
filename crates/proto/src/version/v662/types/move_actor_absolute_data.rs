use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_core::Vec3;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct MoveActorAbsoluteData {
    pub actor_runtime_id: ActorRuntimeID,
    pub header: i8,
    #[endianness(le)]
    pub position: Vec3<f32>,
    pub rotation_x: i8,
    pub rotation_y: i8,
    pub rotation_y_head: i8,
}