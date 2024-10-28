use crate::version::v729::types::ability_data::AbilityData;
use crate::version::v729::types::actor_link::ActorLink;
use crate::version::v729::types::item_stack_descriptor::ItemStackDescriptor;
use crate::version::v729::types::property_sync_data::PropertySyncData;
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::world::gamemode::Gamemode;
use uuid::Uuid;

#[gamepacket(id = 12)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddPlayerPacket {
    pub uuid: Uuid,
    pub name: String,
    pub runtime_id: ActorRuntimeID,
    pub platform_chat_id: String,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    #[endianness(le)]
    pub rotation: Vec3<f32>,
    pub carried_item: ItemStackDescriptor,
    pub gamemode: Gamemode,
    // TODO: Impl SyncedActorDataEntityWrapper
    pub synced_properties: PropertySyncData,
    pub abilities: AbilityData,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub links: Vec<ActorLink>,
}
