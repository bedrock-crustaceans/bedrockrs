use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::PositionTrackingId;

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