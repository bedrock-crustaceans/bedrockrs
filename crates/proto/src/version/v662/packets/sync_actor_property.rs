use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 165)]
#[derive(ProtoCodec)]
pub struct SyncActorPropertyPacket {
    pub property_data: CompoundTag,
}