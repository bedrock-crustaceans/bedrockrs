use crate::version::v729::types::play_status::PlayStatusType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 2)]
#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct PlayStatusPacket {
    pub status: PlayStatusType,
}
