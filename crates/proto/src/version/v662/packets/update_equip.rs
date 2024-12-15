use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{ContainerID, ContainerType};
use crate::version::v662::types::{ActorUniqueID, CompoundTag};

#[gamepacket(id = 81)]
#[derive(ProtoCodec)]
pub struct UpdateEquipPacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    #[endianness(var)]
    pub size: i32,
    pub target_actor_id: ActorUniqueID,
    pub data_tags: CompoundTag,
}