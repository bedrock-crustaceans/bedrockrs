use crate::version::v662::enums::PlayStatus;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 2)]
#[derive(ProtoCodec)]
pub struct PlayStatusPacket {
    pub status: PlayStatus
}