use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_nbt as nbt;
use bedrockrs_proto_derive::ProtoCodec;
use uuid::Uuid;

use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::gamemode::Gamemode;

use crate::types::level_settings::LevelSettings;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_settings::PlayerMovementSettings;

#[derive(ProtoCodec, Debug, Clone)]
pub struct StartGamePacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub actor_game_type: Gamemode,
    pub position: Vec3<LE<f32>>,
    pub rotation: Vec2<LE<f32>>,
    pub settings: LevelSettings,
    pub level_id: String,
    pub level_name: String,
    pub template_content_identity: String,
    pub trial: bool,
    pub movement_settings: PlayerMovementSettings,
    pub current_level_time: LE<u64>,
    pub enchantment_seed: VAR<i32>,
    // TODO Add real value
    #[len_repr(VAR::<u32>)]
    pub block_properties: Vec<u8>,
    // TODO Add real value
    #[len_repr(VAR::<u32>)]
    pub items: Vec<u8>,
    pub multiplayer_correlation_id: String,
    pub enable_item_stack_net_manager: bool,
    pub server_version: String,
    // TODO: This can now be a concrete type rather than an NBT value.
    // How should we do this with the ProtoCodec macro?
    pub player_property_data: nbt::Value,
    pub block_type_registry_checksum: LE<u64>,
    pub world_template_id: Uuid,
    pub enable_clientside_world_generation: bool,
    pub use_block_network_id_hashes: bool,
    pub network_permission: NetworkPermissions,
}
