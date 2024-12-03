use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::BookEditAction;

#[gamepacket(id = 97)]
#[derive(ProtoCodec)]
pub struct BookEditPacket {
    pub action: BookEditAction,
    pub book_slot: i8,
}

// TODO: custom proto impl, enum variants