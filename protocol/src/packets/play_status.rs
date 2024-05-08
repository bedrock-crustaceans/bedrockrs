use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};
use crate::types::play_status::PlayStatusType;

#[derive(Debug, Copy, Clone, MCProtoSerialize, MCProtoDeserialize)]
pub struct PlayStatusPacket {
    pub status: PlayStatusType
}