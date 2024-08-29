use bedrockrs_core::int::LE;
use bedrockrs_core::Vec3;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use crate::types::respawn_state::RespawnState;

#[gamepacket(id = 45)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct RespawnPacket {
    position: Vec3<LE<f32>>,
    state: RespawnState,
    runtime_id: ActorRuntimeID
}
