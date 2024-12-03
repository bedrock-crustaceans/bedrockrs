use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PlayStatus;

#[gamepacket(id = 2)]
#[derive(ProtoCodec)]
pub struct PlayStatusPacket {
    pub status: PlayStatus
}