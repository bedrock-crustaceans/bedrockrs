use uuid::Uuid;
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::world::gamemode::Gamemode;
use crate::version::v662::types::{ItemStackDescriptor};

#[gamepacket(id = 9)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddPlayerPacket {
    pub uuid: Uuid,
    pub username: String,
    pub target_runtime_id: ActorRuntimeID,
    pub platform_chat_id: String,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    #[endianness(le)]
    pub rotation: Vec2<f32>,
    #[endianness(le)]
    pub head_yaw: f32,
    pub carried_item: ItemStackDescriptor,
    pub gamemode: Gamemode,
    //pub actor_meta_data: ActorMetaData,
}
