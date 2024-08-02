use bedrockrs_core::int::{VAR, LE};
use bedrockrs_proto_derive::ProtoCodec;
use crate::types::network_block_pos::NetworkBlockPos;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerActionPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub action: VAR<i32>,
    pub block_pos: NetworkBlockPos,
    pub result_pos: NetworkBlockPos,
    pub face: VAR<i32>
}