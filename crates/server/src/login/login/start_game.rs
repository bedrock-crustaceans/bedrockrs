use bedrockrs_core::{Vec2, Vec3};
use std::collections::HashMap;
use uuid::Uuid;

use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::difficulty::Difficulty;
use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_shared::world::gamemode::Gamemode;
use bedrockrs_shared::world::generator_type::GeneratorType;

use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepackets::GamePackets;
use crate::login::provider::{LoginProvider, LoginProviderStatus};
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::start_game::StartGamePacket;
use crate::types::base_game_version::BaseGameVersion;
use crate::types::block_pos::BlockPos;
use crate::types::chat_restriction_level::ChatRestrictionLevel;
use crate::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::types::experiments::Experiments;
use crate::types::level_settings::LevelSettings;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::play_status::PlayStatusType;
use crate::types::player_movement_mode::PlayerMovementMode;
use crate::types::player_movement_settings::PlayerMovementSettings;
use crate::types::spawn_biome_type::SpawnBiomeType;
use crate::types::spawn_settings::SpawnSettings;
use bedrockrs_shared::world::editor_world_type::EditorWorldType;

pub async fn start_game(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> Result<LoginProviderStatus, LoginError> {
    //////////////////////////////////////
    // Start Game Packet
    //////////////////////////////////////

    let start_game = StartGamePacket {
        target_actor_id: ActorUniqueID(609),
        target_runtime_id: ActorRuntimeID(402),
        gamemode: Gamemode::Creative,
        position: Vec3 {
            x: 4.0,
            y: 6.0,
            z: 7.0,
        },
        rotation: Vec2 { x: 270.0, y: 90.0 },
        settings: LevelSettings {
            seed: 777777777777,
            spawn_settings: SpawnSettings {
                biome_type: SpawnBiomeType::Default,
                user_defined_biome_name: String::from("RandomBiome"),
                dimension: Dimension::Overworld,
            },
            generator_type: GeneratorType::Overworld,
            gamemode: Gamemode::Creative,
            hardcore: false,
            difficulty: Difficulty::Peaceful,
            default_spawn_block: BlockPos {
                x: 100,
                y: 200,
                z: 300,
            },
            achievements_disabled: true,
            editor_world_type: EditorWorldType::NotEditor,
            created_in_editor: false,
            exported_from_editor: false,
            day_cycle_stop_time: 2000,
            education_edition_offer: 0,
            education_features: false,
            education_product_id: String::from(""),
            rain_level: 300.0,
            lightning_level: 400.0,
            platform_locked_content: false,
            multiplayer_intended: true,
            lan_broadcasting_intended: true,
            broadcasting_settings_xbox_live: 2,
            broadcasting_settings_platform: 2,
            commands_enabled: true,
            texture_pack_required: false,
            gamerules: vec![],
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            bonus_chest: false,
            start_with_map: false,
            player_permission: 3,
            server_chunk_tick_radius: 4,
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
            limited_world_width: 0,
            limited_world_depth: 0,
            new_nether: true,
            edu_shared_uri_resource: EduSharedResourceUri {
                button_name: String::from(""),
                link_uri: String::from(""),
            },
            force_experimental_gameplay: true,
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
            server_id: String::from(""),
            world_id: String::from(""),
            scenario_id: String::from(""),
        },
        level_id: String::from("UmFuZG9tIFdvcmxk"),
        level_name: String::from("Random World"),
        template_content_identity: String::new(),
        trial: false,
        movement_settings: PlayerMovementSettings {
            authority_mode: PlayerMovementMode::Client,
            rewind_history_size: 3200,
            server_authoritative_block_breaking: false,
        },
        current_level_time: 9000,
        enchantment_seed: 99000,
        blocks: vec![],
        items: vec![],
        multiplayer_correlation_id: String::from("c5d3d2cc-27fd-4221-9de6-d22c4d423d53"),
        enable_item_stack_net_manager: false,
        server_version: String::from("1.19.2"),
        player_property_data: nbtx::Value::Compound(HashMap::new()),
        block_type_registry_checksum: 0,
        world_template_id: Uuid::nil(),
        enable_clientside_world_generation: false,
        use_block_network_id_hashes: true,
        network_permission: NetworkPermissions {
            server_auth_sound_enabled: false,
        },
    };

    conn.send(GamePackets::StartGame(start_game)).await?;
    conn.send(GamePackets::PlayStatus(PlayStatusPacket {
        status: PlayStatusType::PlayerSpawn,
    }))
    .await?;

    conn.flush().await?;

    Ok(())
}
