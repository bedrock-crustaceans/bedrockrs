use crate::types::respawn_state::RespawnState;
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[gamepacket(id = 45)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct RespawnPacket {
    #[endianness(le)]
    pub position: Vec3<f32>,
    pub state: RespawnState,
    pub runtime_id: ActorRuntimeID,
}
