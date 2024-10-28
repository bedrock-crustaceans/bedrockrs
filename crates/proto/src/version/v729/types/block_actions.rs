use crate::version::v729::types::block_pos::BlockPos;
use bedrockrs_macros::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct BlockActions {
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    actions: Vec<BlockActionData>,
}

#[derive(Debug, Clone, ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum BlockActionType {
    StartBreak = 0,
    AbortBreak = 1,
    StopBreak = 2,
    GetUpdatedBlock = 3,
    DropItem = 4,
    StartSleeping = 5,
    StopSleeping = 6,
    Respawn = 7,
    Jump = 8,
    StartSprint = 9,
    StopSprint = 10,
    StartSneak = 11,
    StopSneak = 12,
    CreativePlayerDestroyBlock = 13,
    DimensionChangeDone = 14,
    StartGlide = 15,
    StopGlide = 16,
    BuildDenied = 17,
    CrackBreak = 18,
    ChangeSkin = 19,
    SetEnchantmentSeed = 20,
    StartSwimming = 21,
    StopSwimming = 22,
    StartSpinAttack = 23,
    StopSpinAttack = 24,
    StartBuildingBlock = 25,
    PredictDestroyBlock = 26,
    ContinueDestroyBlock = 27,
    StartItemUseOn = 28,
    StopItemUseOn = 29,
    HandledTeleport = 30,
    MissedSwing = 31,
    StartCrawling = 32,
    StopCrawling = 33,
    StartFlying = 34,
    StopFlying = 35,
    ClientAckServerData = 36,
}

#[derive(Debug, Clone, ProtoCodec)]
pub struct BlockActionData {
    action: BlockActionType,
    block_pos: BlockPos,
    #[endianness(var)]
    face: i32,
}
