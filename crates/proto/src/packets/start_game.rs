use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use uuid::Uuid;

use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::gamemode::Gamemode;
use crate::types::block_entry::BlockEntry;
use crate::types::item_entry::ItemEntry;
use crate::types::level_settings::LevelSettings;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_settings::PlayerMovementSettings;

#[gamepacket(id = 11)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct StartGamePacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub gamemode: Gamemode,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub rotation: Vec2<f32>,
    pub settings: LevelSettings,
    pub level_id: String,
    pub level_name: String,
    pub template_content_identity: String,
    pub trial: bool,
    pub movement_settings: PlayerMovementSettings,
    #[endianness(le)]
    pub current_level_time: u64,
    #[endianness(var)]
    pub enchantment_seed: i32,
    /// List of all custom blocks registered on the server.
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub blocks: Vec<BlockEntry>,
    /// List of all items with their legacy IDs that are available in the game.
    /// Failing to send any of the items that are in the game will crash mobile clients.
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub items: Vec<ItemEntry>,
    pub multiplayer_correlation_id: String,
    pub enable_item_stack_net_manager: bool,
    pub server_version: String,
    // TODO: This can now be a concrete type rather than an NBT value.
    // How should we do this with the ProtoCodec macro?
    #[nbt]
    pub player_property_data: nbtx::Value,
    #[endianness(le)]
    pub block_type_registry_checksum: u64,
    pub world_template_id: Uuid,
    pub enable_clientside_world_generation: bool,
    pub use_block_network_id_hashes: bool,
    pub network_permission: NetworkPermissions,
}
