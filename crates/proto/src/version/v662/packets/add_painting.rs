use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, ActorUniqueID, Vec3};

#[gamepacket(id = 22)]
#[derive(ProtoCodec)]
pub struct AddPaintingPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub position: Vec3,
    #[endianness(var)]
    pub direction: i32,
    pub motif: String,
}