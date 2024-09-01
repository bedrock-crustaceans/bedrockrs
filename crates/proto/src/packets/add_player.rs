use crate::types::ability_data::AbilityData;
use crate::types::actor_link::ActorLink;
use crate::types::network_item_stack_descriptor::NetworkItemStackDescriptor;
use crate::types::property_sync_data::PropertySyncData;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::Vec3;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::world::gamemode::Gamemode;
use uuid::Uuid;

#[gamepacket(id = 12)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddPlayerPacket {
    uuid: Uuid,
    name: String,
    runtime_id: ActorRuntimeID,
    platform_chat_id: String,
    position: Vec3<LE<f32>>,
    velocity: Vec3<LE<f32>>,
    rotation: Vec3<LE<f32>>,
    carried_item: NetworkItemStackDescriptor,
    gamemode: Gamemode,
    // TODO: Impl SyncedActorDataEntityWrapper
    synced_properties: PropertySyncData,
    abilities: AbilityData,
    #[len_repr(VAR::<u32>)]
    links: Vec<ActorLink>,
}
