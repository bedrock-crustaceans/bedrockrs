use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::difficulty::Difficulty;

#[gamepacket(id = 60)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct UpdateDifficultyPacket {
    pub difficulty: Difficulty,
}
