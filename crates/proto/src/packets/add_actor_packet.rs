use bedrockrs_core::{
    int::{LE, VAR},
    Vec2, Vec3,
};
use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_shared::{actor_runtime_id::ActorRuntimeID, actor_unique_id::ActorUniqueID};

use crate::types::{
    actor_link::ActorLinkList, actor_type::ActorType, attribute::AttributeList, dataitem::DataItemList, property_sync_data::PropertySyncData
};

#[derive(ProtoCodec, Debug, Clone)]
pub struct AddActorPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub actor_type: ActorType,
    pub position: Vec3<LE<f32>>,
    pub velocity: Vec3<LE<f32>>,
    pub rotation: Vec2<LE<f32>>,
    pub y_head_rotation: LE<f32>,
    pub y_body_rotation: LE<f32>,
    pub attributes: AttributeList,
    pub actor_data: DataItemList,
    pub synched_properties: PropertySyncData,
    pub actor_links: ActorLinkList,
}
