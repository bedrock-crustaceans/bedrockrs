use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{CompoundTag, PositionTrackingId};

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum Action {
    Update = 0,
    Destroy = 1,
    NotFound = 2,
}

#[gamepacket(id = 153)]
#[derive(ProtoCodec)]
pub struct PositionTrackingDBServerBroadcastPacket {
    pub action: Action,
    pub id: PositionTrackingId,
    pub position_tracking_data: CompoundTag,
}