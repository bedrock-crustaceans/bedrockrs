use bedrockrs_core::int::{VAR, LE};
use bedrockrs_proto_derive::ProtoCodec;
use crate::types::network_block_pos::NetworkBlockPos;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

pub enum PlayerActionType {
    Unknown = -1,
    StartDestroyBlock = 0,
    AbortDestroyBlock,
    StopDestroyBlock,
    GetUpdatedBlock,
    DropItem,
    StartSleeping,
    StopSleeping,
    Respawn,
    StartJump,
    StartSprinting,
    StopSprinting,
    StartSneaking,
    StopSneaking,
    CreativeDestroyBlock,
    ChangeDimensionAck,
    StartGliding,
    StopGlibiding,
    DenyDestroyBlock,
    CrackBlock,
    ChangeSkin,
    DeprecatedUpdatedEnchantingSeed,
    StartSwimming,
    StopSwimming,
    StartSpinAttack,
    StopSpinAttack,
    InteractWithBlock,
    PredictDestroyBlock,
    ContinueDestroyBlock,
    StartItemUseOn,
    StopItemUseOn,
    HandledTeleport,
    MissedSwing,
    StartCrawling,
    StopCrawling,
    StartFlying,
    StopFlying,
    ClientAckServerData,
    Count
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerActionPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub action: VAR<i32>,
    pub block_pos: NetworkBlockPos,
    pub result_pos: NetworkBlockPos,
    pub face: VAR<i32>
}