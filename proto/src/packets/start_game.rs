use bedrock_core::{ActorRuntimeID, ActorUniqueID, LE, Uuid, VAR, Vec2, Vec3};
use bedrock_core::gamemode::Gamemode;
use nbt::NbtTag;
use proto_derive::ProtoCodec;
use crate::types::level_settings::LevelSettings;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_settings::PlayerMovementSettings;

#[derive(ProtoCodec, Debug, Clone)]
pub struct StartGamePacket {
    target_actor_id: ActorUniqueID,
    target_runtime_id: ActorRuntimeID,
    actor_game_type: Gamemode,
    position: Vec3,
    rotation: Vec2,
    settings: LevelSettings,
    level_id: String,
    level_name: String,
    template_content_identity: String,
    trial: bool,
    movement_settings: PlayerMovementSettings,
    current_level_time: LE<u64>,
    enchantment_seed: VAR<i32>,
    // TODO Add real value
    #[len_type(VAR::<u32>)]
    block_properties: Vec<u8>,
    // TODO Add real value
    #[len_type(VAR::<u32>)]
    items: Vec<u8>,
    multiplayer_correlation_id: String,
    enable_item_stack_net_manager: bool,
    server_version: String,
    player_property_data: NbtTag,
    block_type_registry_checksum: LE<u64>,
    world_template_id: Uuid,
    enable_clientside_world_generation: bool,
    use_block_network_id_hashes: bool,
    network_permission: NetworkPermissions,
}
