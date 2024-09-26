use crate::types::ability_data::AbilityData;
use crate::types::actor_link::ActorLink;
use crate::types::network_item_stack_descriptor::NetworkItemStackDescriptor;
use crate::types::property_sync_data::PropertySyncData;
use bedrockrs_core::int::{LE, VAR};
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
    pub position: Vec3<LE<f32>>,
    pub velocity: Vec3<LE<f32>>,
    pub rotation: Vec3<LE<f32>>,
    pub carried_item: NetworkItemStackDescriptor,
    pub gamemode: Gamemode,
    // TODO: Impl SyncedActorDataEntityWrapper
    pub synced_properties: PropertySyncData,
    pub abilities: AbilityData,
    #[len_repr(VAR::<u32>)]
    pub links: Vec<ActorLink>,
}
