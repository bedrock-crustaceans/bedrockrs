use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ScorePacketType;

#[gamepacket(id = 108)]
#[derive(ProtoCodec)]
pub struct SetScorePacket {
    pub score_packet_type: ScorePacketType,
}

// TODO: this kinda sucks, might wanna refactor later