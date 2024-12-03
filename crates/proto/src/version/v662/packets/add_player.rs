use uuid::Uuid;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{BuildPlatform, GameType};
use crate::version::v662::types::{ActorLink, ActorRuntimeID, DataItem, NetworkItemStackDescriptor, PropertySyncData, SerializedAbilitiesData, Vec2, Vec3};

#[gamepacket(id = 12)]
#[derive(ProtoCodec)]
pub struct AddPlayerPacket {
    pub uuid: Uuid,
    pub player_name: String,
    pub target_runtime_id: ActorRuntimeID,
    pub platform_chat_id: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Vec2,
    #[endianness(le)]
    pub y_head_rotation: f32,
    pub carried_item: NetworkItemStackDescriptor,
    pub player_game_type: GameType,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entity_data: Vec<DataItem>, // TODO: Verify vec_repr & vec_endianness
    pub synced_properties: PropertySyncData,
    pub abilities_data: SerializedAbilitiesData,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actor_links: Vec<ActorLink>,
    pub device_id: String,
    pub build_platform: BuildPlatform,
}