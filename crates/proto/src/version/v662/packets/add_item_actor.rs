use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, ActorUniqueID, DataItem, NetworkItemStackDescriptor, Vec3};

#[gamepacket(id = 15)]
#[derive(ProtoCodec)]
pub struct AddItemActorPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub item: NetworkItemStackDescriptor,
    pub position: Vec3,
    pub velocity: Vec3,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entity_data: Vec<DataItem>, // TODO: Verify vec_repr & vec_endianness
    pub from_fishing: bool,
}