use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::types::play_status::PlayStatusType;

#[gamepacket(id = 2)]
#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct PlayStatusPacket {
    pub status: PlayStatusType,
}
