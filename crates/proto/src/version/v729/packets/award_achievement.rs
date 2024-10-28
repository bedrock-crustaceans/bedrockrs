use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 309)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AwardAchievementPacket {
    /// Refers to the id of the achievement.
    // TODO: Turn into enum if possible
    #[endianness(le)]
    id: i32,
}
