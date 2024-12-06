use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::LevelEvent;
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 124)]
#[derive(ProtoCodec)]
pub struct LevelEventGenericPacket {
    pub event_id: LevelEvent,
    pub event_data: CompoundTag,
}