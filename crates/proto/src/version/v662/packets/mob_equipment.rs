use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ContainerID;
use crate::version::v662::types::{ActorRuntimeID, NetworkItemStackDescriptor};

#[gamepacket(id = 31)]
#[derive(ProtoCodec)]
pub struct MobEquipmentPacket {
    pub target_runtime_id: ActorRuntimeID,
    pub item: NetworkItemStackDescriptor,
    pub slot: i8,
    pub selected_slot: i8,
    pub container_id: ContainerID,
}