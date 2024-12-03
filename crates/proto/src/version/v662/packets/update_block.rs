use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 21)]
#[derive(ProtoCodec)]
pub struct UpdateBlockPacket {
    pub block_position: NetworkBlockPosition,
    #[endianness(var)]
    pub block_runtime_id: u32,
    #[endianness(var)]
    pub flags: u32,
    #[endianness(var)]
    pub layer: u32,
}