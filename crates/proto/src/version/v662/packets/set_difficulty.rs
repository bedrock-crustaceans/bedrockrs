use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::Difficulty;

#[gamepacket(id = 60)]
#[derive(ProtoCodec)]
pub struct SetDifficultyPacket {
    pub difficulty: Difficulty,
}