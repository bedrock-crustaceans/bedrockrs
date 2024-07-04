use bedrock_core::{ActorRuntimeID, ActorUniqueID, Difficulty, Dimension, GeneratorType, LE, Uuid, VAR, Vec2, Vec3};
use bedrock_core::gamemode::Gamemode;
use nbt::NbtTag;
use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::start_game::StartGamePacket;
use crate::types::base_game_version::BaseGameVersion;
use crate::types::chat_restriction_level::ChatRestrictionLevel;
use crate::types::editor_world_type::EditorWorldType;
use crate::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::types::experiments::Experiments;
use crate::types::level_settings::LevelSettings;
use crate::types::network_block_pos::NetworkBlockPos;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_mode::PlayerMovementMode;
use crate::types::player_movement_settings::PlayerMovementSettings;
use crate::types::spawn_biome_type::SpawnBiomeType;
use crate::types::spawn_settings::SpawnSettings;

pub async fn start_game(conn: &mut ConnectionShard, provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    // Start Game Packet
    //////////////////////////////////////

    let start_game = StartGamePacket{
        target_actor_id: ActorUniqueID(1),
        target_runtime_id: ActorRuntimeID(1),
        actor_game_type: Gamemode::Survival,
        position: Vec3 { x: 0, y: 0, z: 0 },
        rotation: Vec2 { x: 0, z: 0 },
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
                z: VAR::new(0)
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
            multiplayer_intended: false,
            lan_broadcasting_intended: false,
            broadcasting_settings_xbox_live: VAR::new(0),
            broadcasting_settings_platform: VAR::new(0),
            commands_enabled: false,
            texture_pack_required: false,
            rules: vec![],
            experiments: Experiments { experiments: vec![], ever_toggled: false },
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
            base_game_version: BaseGameVersion(String::from("1.21.0")),
            limited_world_width: LE::new(0),
            limited_world_depth: LE::new(0),
            new_nether: true,
            edu_shared_uri_resource: EduSharedResourceUri { button_name: String::from(""), link_uri: String::from("") },
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
        player_property_data: NbtTag::Empty,
        block_type_registry_checksum: LE::new(0),
        world_template_id: Uuid::new_v4(),
        enable_clientside_world_generation: false,
        use_block_network_id_hashes: false,
        network_permission: NetworkPermissions {
            server_auth_sound_enabled: false,
        },
    };

    match conn.send(GamePacket::StartGame(start_game)).await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    };

    Ok(())
}
