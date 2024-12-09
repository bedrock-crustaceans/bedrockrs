use crate::version::v662::types::PositionTrackingId;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum Action {
    Query = 0
}

#[gamepacket(id = 154)]
#[derive(ProtoCodec)]
pub struct PositionTrackingDBClientRequestPacket {
    pub action: Action,
    pub id: PositionTrackingId,
}