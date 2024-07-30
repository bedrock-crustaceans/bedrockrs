use std::collections::HashMap;

use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_nbt::NbtTag;
use uuid::Uuid;

use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::difficulty::Difficulty;
use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_shared::world::gamemode::Gamemode;
use bedrockrs_shared::world::generator_type::GeneratorType;

use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::LoginProviderServer;
use crate::packets::start_game::StartGamePacket;
use crate::types::base_game_version::BaseGameVersion;
use crate::types::chat_restriction_level::ChatRestrictionLevel;
use crate::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::types::experiments::Experiments;
use crate::types::level_settings::LevelSettings;
use crate::types::network_block_pos::NetworkBlockPos;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_mode::PlayerMovementMode;
use crate::types::player_movement_settings::PlayerMovementSettings;
use crate::types::spawn_biome_type::SpawnBiomeType;
use crate::types::spawn_settings::SpawnSettings;
use bedrockrs_shared::world::editor_world_type::EditorWorldType;

pub async fn start_game(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    // Start Game Packet
    //////////////////////////////////////

    let start_game = StartGamePacket {
        target_actor_id: ActorUniqueID(0),
        target_runtime_id: ActorRuntimeID(0),
        actor_game_type: Gamemode::Survival,
        position: Vec3 {
            x: LE::new(0.0),
            y: LE::new(0.0),
            z: LE::new(0.0),
        },
        rotation: Vec2 {
            x: LE::new(0.0),
            y: LE::new(0.0),
        },
        settings: LevelSettings {
            seed: LE::new(80085),
            spawn_settings: SpawnSettings {
                biome_type: SpawnBiomeType::Default,
                user_defined_biome_name: String::from(""),
                dimension: Dimension::Overworld,
            },
            generator_type: GeneratorType::Overworld,
            game_type: Gamemode::Survival,
            hardcore: false,
            difficulty: Difficulty::Peaceful,
            default_spawn_block: NetworkBlockPos {
                x: VAR::new(0),
                y: VAR::new(0),
                z: VAR::new(0),
            },
            achievements_disabled: false,
            editor_world_type: EditorWorldType::NotEditor,
            created_in_editor: false,
            exported_from_editor: false,
            day_cycle_stop_time: VAR::new(0),
            education_edition_offer: VAR::new(0),
            education_features: false,
            education_product_id: String::from(""),
            rain_level: LE::new(0.0),
            lightning_level: LE::new(0.0),
            platform_locked_content: false,
            multiplayer_intended: true,
            lan_broadcasting_intended: true,
            broadcasting_settings_xbox_live: VAR::new(4),
            broadcasting_settings_platform: VAR::new(4),
            commands_enabled: false,
            texture_pack_required: false,
            gamerules: vec![],
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            bonus_chest: false,
            start_with_map: false,
            player_permission: VAR::new(0),
            server_chunk_tick_radius: LE::new(4),
            locked_behavior_packs: false,
            locked_resource_packs: false,
            from_locked_template: false,
            msa_gamertags_only: false,
            from_template: false,
            is_template_locked_settings: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: BaseGameVersion(String::from("*")),
            limited_world_width: LE::new(16),
            limited_world_depth: LE::new(16),
            new_nether: true,
            edu_shared_uri_resource: EduSharedResourceUri {
                button_name: String::from(""),
                link_uri: String::from(""),
            },
            force_experimental_gameplay: Some(false),
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
            server_id: String::from(""),
            world_id: String::from(""),
            scenario_id: String::from(""),
        },
        level_id: String::from(""),
        level_name: String::from(""),
        template_content_identity: String::from(""),
        trial: false,
        movement_settings: PlayerMovementSettings {
            authority_mode: PlayerMovementMode::Client,
            rewind_history_size: VAR::new(0),
            server_authoritative_block_breaking: false,
        },
        current_level_time: LE::new(12000),
        enchantment_seed: VAR::new(80085),
        block_properties: vec![],
        items: vec![],
        multiplayer_correlation_id: String::from(""),
        enable_item_stack_net_manager: false,
        server_version: String::from("1.21.0"),
        player_property_data: NbtTag::Compound(HashMap::new()),
        block_type_registry_checksum: LE::new(0),
        world_template_id: Uuid::new_v4(),
        enable_clientside_world_generation: false,
        use_block_network_id_hashes: false,
        network_permission: NetworkPermissions {
            server_auth_sound_enabled: false,
        },
    };

    conn.send(GamePacket::StartGame(start_game)).await.map_err(|e| LoginError::ConnectionError(e))?;
    conn.flush().await.map_err(|e| LoginError::ConnectionError(e))?;

    Ok(())
}
