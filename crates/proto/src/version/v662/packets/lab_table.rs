use crate::version::v662::enums::LabTableReactionType;
use crate::version::v662::types::BlockPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum Type {
    StartCombine = 0,
    StartReaction = 1,
    Reset = 2,
}

#[gamepacket(id = 109)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct LabTablePacket {
    pub lab_table_packet_type: Type,
    pub position: BlockPos,
    pub reaction: LabTableReactionType,
}