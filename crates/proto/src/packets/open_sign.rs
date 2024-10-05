use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::types::block_pos::BlockPos;

#[gamepacket(id = 303)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct OpenSignPacket {
    pos: BlockPos,
    front: bool,
}