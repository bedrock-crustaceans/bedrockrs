use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::BlockPos;

#[gamepacket(id = 34)]
#[derive(ProtoCodec)]
pub struct BlockPickRequestPacket {
    pub position: BlockPos,
    pub with_data: bool,
    pub max_slots: i8,
}