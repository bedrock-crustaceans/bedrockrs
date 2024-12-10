use crate::version::v662::types::{ActorRuntimeID, ActorUniqueID};
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 22)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct AddPaintingPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub direction: i32,
    pub motif: String,
}