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
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    #[endianness(le)]
    pub rotation: Vec2<f32>,
    #[endianness(le)]
    pub y_head_rotation: f32,
    #[endianness(le)]
    pub y_body_rotation: f32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub attributes: Vec<Attribute>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actor_data: Vec<DataItem>,
    pub synced_properties: PropertySyncData,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actor_links: Vec<ActorLink>,
}
