use crate::version::v729::types::block_pos::BlockPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 303)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct OpenSignPacket {
    pos: BlockPos,
    front: bool,
}
