use bedrockrs_core::{
    int::{LE, VAR},
    Vec2, Vec3,
};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::{actor_runtime_id::ActorRuntimeID, actor_unique_id::ActorUniqueID};

use crate::types::{
    actor_link::ActorLink, attribute::Attribute, data_item::DataItem,
    property_sync_data::PropertySyncData,
};

#[gamepacket(id = 12)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddActorPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub actor_type: String,
    pub position: Vec3<LE<f32>>,
    pub velocity: Vec3<LE<f32>>,
    pub rotation: Vec2<LE<f32>>,
    pub y_head_rotation: LE<f32>,
    pub y_body_rotation: LE<f32>,
    #[len_repr(VAR::<u32>)]
    pub attributes: Vec<Attribute>,
    #[len_repr(VAR::<u32>)]
    pub actor_data: Vec<DataItem>,
    pub synced_properties: PropertySyncData,
    #[len_repr(VAR::<u32>)]
    pub actor_links: Vec<ActorLink>,
}
