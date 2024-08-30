use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::difficulty::Difficulty;

#[gamepacket(id = 60)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetDifficultyPacket {
    difficulty: Difficulty,
}
