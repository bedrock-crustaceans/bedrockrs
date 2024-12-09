use crate::version::v662::types::GameRulesChangedPacketData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 72)]
#[derive(ProtoCodec)]
pub struct GameRulesChangedPacket {
    pub rules_data: GameRulesChangedPacketData,
}