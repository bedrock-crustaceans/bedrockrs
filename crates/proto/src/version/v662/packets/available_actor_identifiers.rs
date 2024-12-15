use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 119)]
#[derive(ProtoCodec)]
pub struct AvailableActorIdentifiersPacket {
    pub actor_info_list: CompoundTag,
}