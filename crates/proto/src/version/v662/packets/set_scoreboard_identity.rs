use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ScoreboardIdentityPacketType;

#[gamepacket(id = 112)]
#[derive(ProtoCodec)]
pub struct SetScoreboardIdentityPacket {
    pub scoreboard_identity_packet_type: ScoreboardIdentityPacketType,
}

// TODO: same thing here, scoreboard seem to be a bit janky. Might refactor