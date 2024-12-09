use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::LevelEvent;

#[gamepacket(id = 124)]
#[derive(ProtoCodec)]
pub struct LevelEventGenericPacket {
    pub event_id: LevelEvent,
    #[nbt]
    pub event_data: nbtx::Value, // TODO: NBT Structure
}