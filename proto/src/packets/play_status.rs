use bedrockrs_proto_derive::ProtoCodec;

use crate::types::play_status::PlayStatusType;

#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct PlayStatusPacket {
    pub status: PlayStatusType,
}
