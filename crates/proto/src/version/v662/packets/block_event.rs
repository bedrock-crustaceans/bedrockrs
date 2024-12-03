use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 26)]
#[derive(ProtoCodec)]
pub struct BlockEventPacket  {
    pub block_position: NetworkBlockPosition,
    #[endianness(var)]
    pub event_type: i32,
    #[endianness(var)]
    pub event_value: i32,
}