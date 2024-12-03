use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, NetworkItemStackDescriptor};

#[gamepacket(id = 32)]
#[derive(ProtoCodec)]
pub struct MobArmorEquipmentPacket {
    pub target_runtime_id: ActorRuntimeID,
    pub head: NetworkItemStackDescriptor,
    pub torso: NetworkItemStackDescriptor,
    pub legs: NetworkItemStackDescriptor,
    pub feet: NetworkItemStackDescriptor,
}