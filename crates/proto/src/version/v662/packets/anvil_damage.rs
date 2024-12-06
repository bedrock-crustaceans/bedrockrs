use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 141)]
#[derive(ProtoCodec)]
pub struct AnvilDamagePacket {
    pub damage_amount: i8,
    pub block_position: NetworkBlockPosition,
}