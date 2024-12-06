use uuid::Uuid;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::GameType;
use crate::version::v662::types::{ActorRuntimeID, ActorUniqueID, CompoundTag, ItemData, LevelSettings, NetworkPermissions, SyncedPlayerMovementSettings, Vec2, Vec3};

#[derive(ProtoCodec)]
struct BlockProperty {
    pub block_name: String,
    pub block_definition: CompoundTag,
}

#[gamepacket(id = 11)]
#[derive(ProtoCodec)]
pub struct StartGamePacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub actor_game_type: GameType,
    pub position: Vec3,
    pub rotation: Vec2,
    pub settings: LevelSettings,
    pub level_id: String,
    pub level_name: String,
    pub template_content_identity: String,
    pub is_trial: bool,
    pub movement_settings: SyncedPlayerMovementSettings,
    #[endianness(le)]
    pub current_level_time: u64,
    #[endianness(var)]
    pub enchantment_seed: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub block_properties: Vec<BlockProperty>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub item_list: Vec<ItemData>,
    pub multiplayer_correlation_id: String,
    pub enable_item_stack_net_manager: bool,
    pub server_version: String,
    pub player_property_data: CompoundTag,
    #[endianness(le)]
    pub server_block_type_registry_checksum: u64,
    pub world_template_id: Uuid,
    pub server_enabled_client_side_generation: bool,
    pub block_network_ids_are_hashes: bool,
    pub network_permissions: NetworkPermissions,
}
