use crate::types::{block_pos::BlockPos, player_action_type::PlayerActionType};
use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[gamepacket(id = 36)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerActionPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub action: PlayerActionType,
    pub block_pos: BlockPos,
    pub result_pos: BlockPos,
    pub face: VAR<i32>,
}
