use bedrockrs_macros::ProtoCodec;

use bedrockrs_shared::world::difficulty::Difficulty;
use bedrockrs_shared::world::gamemode::Gamemode;
use bedrockrs_shared::world::generator_type::GeneratorType;

use crate::version::v729::types::base_game_version::BaseGameVersion;
use crate::version::v729::types::block_pos::BlockPos;
use crate::version::v729::types::chat_restriction_level::ChatRestrictionLevel;
use crate::version::v729::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::version::v729::types::experiments::Experiments;
use crate::version::v729::types::gamerule::GameRule;
use crate::version::v729::types::spawn_settings::SpawnSettings;
use bedrockrs_shared::world::editor_world_type::EditorWorldType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct LevelSettings {
    #[endianness(le)]
    pub seed: u64,
    pub spawn_settings: SpawnSettings,
    pub generator_type: GeneratorType,
    pub gamemode: Gamemode,
    pub hardcore: bool,
    pub difficulty: Difficulty,
    pub default_spawn_block: BlockPos,
    pub achievements_disabled: bool,
    pub editor_world_type: EditorWorldType,
    pub created_in_editor: bool,
    pub exported_from_editor: bool,
    #[endianness(var)]
    pub day_cycle_stop_time: i32,
    // TODO: Turn into enum
    #[endianness(var)]
    pub education_edition_offer: i32,
    pub education_features: bool,
    pub education_product_id: String,
    #[endianness(le)]
    pub rain_level: f32,
    #[endianness(le)]
    pub lightning_level: f32,
    pub platform_locked_content: bool,
    pub multiplayer_intended: bool,
    pub lan_broadcasting_intended: bool,
    // TODO turn into enum
    #[endianness(var)]
    pub broadcasting_settings_xbox_live: i32,
    // TODO: Turn into enum
    #[endianness(var)]
    pub broadcasting_settings_platform: i32,
    pub commands: bool,
    pub texture_pack_required: bool,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub gamerules: Vec<GameRule>,
    pub experiments: Experiments,
    pub bonus_chest: bool,
    pub start_with_map: bool,
    // TODO turn into enum
    #[endianness(var)]
    pub player_permission: i32,
    #[endianness(le)]
    pub server_chunk_tick_radius: i32,
    pub locked_behavior_packs: bool,
    pub locked_resource_packs: bool,
    pub from_locked_template: bool,
    pub msa_gamertags_only: bool,
    pub from_template: bool,
    pub is_template_locked_settings: bool,
    pub only_spawn_v1_villagers: bool,
    pub persona_disabled: bool,
    pub custom_skins_disabled: bool,
    pub emote_chat_muted: bool,
    pub base_game_version: BaseGameVersion,
    #[endianness(le)]
    pub limited_world_width: i32,
    #[endianness(le)]
    pub limited_world_depth: i32,
    pub new_nether: bool,
    pub edu_shared_uri_resource: EduSharedResourceUri,
    pub force_experimental_gameplay: Option<bool>,
    pub chat_restriction_level: ChatRestrictionLevel,
    pub disable_player_interactions: bool,
    pub server_id: String,
    pub world_id: String,
    pub scenario_id: String,
}
