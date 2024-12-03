use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::GameRulesChangedPacketData;

#[gamepacket(id = 72)]
#[derive(ProtoCodec)]
pub struct GameRulesChangedPacket {
    pub rules_data: GameRulesChangedPacketData,
}