use crate::version::v662::types::NetworkBlockPosition;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 303)]
#[derive(ProtoCodec)]
pub struct OpenSignPacket {
    pub pos: NetworkBlockPosition,
    pub is_front: bool,
}