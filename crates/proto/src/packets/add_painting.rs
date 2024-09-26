use bedrockrs_core::{
    int::{LE, VAR},
    Vec3,
};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::{actor_runtime_id::ActorRuntimeID, actor_unique_id::ActorUniqueID};

#[gamepacket(id = 22)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddPaintingPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub position: Vec3<LE<f32>>,
    pub direction: VAR<i32>,
    pub motif: String,
}
