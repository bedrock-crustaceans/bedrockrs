use crate::version::v662::types::PositionTrackingId;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum Action {
    Query = 0
}

#[gamepacket(id = 154)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PositionTrackingDBClientRequestPacket {
    pub action: Action,
    pub id: PositionTrackingId,
}