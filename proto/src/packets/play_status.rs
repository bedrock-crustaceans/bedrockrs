use proto_derive::{ProtoCodec};

use crate::types::play_status::PlayStatusType;

#[derive(Debug, Copy, Clone, ProtoCodec)]
pub struct PlayStatusPacket {
    pub status: PlayStatusType,
}
