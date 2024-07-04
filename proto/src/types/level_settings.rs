use bedrock_core::{Difficulty, GeneratorType, LE, VAR, Vec3};
use bedrock_core::gamemode::Gamemode;
use proto_derive::ProtoCodec;
use crate::types::base_game_version::BaseGameVersion;
use crate::types::chat_restriction_level::ChatRestrictionLevel;
use crate::types::editor_world_type::EditorWorldType;
use crate::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::types::experiments::Experiments;
use crate::types::game_rule::GameRule;
use crate::types::spawn_settings::SpawnSettings;

#[derive(ProtoCodec, Debug, Clone)]
pub struct LevelSettings {
    seed: LE<u64>,
    spawn_settings: SpawnSettings,
    generator_type: GeneratorType,
    game_type: Gamemode,
    hardcore: bool,
    difficulty: Difficulty,
    default_spawn_block: Vec3,
    achievements_disabled: bool,
    editor_world_type: EditorWorldType,
    created_in_editor: bool,
    exported_from_editor: bool,
    day_cycle_stop_time: VAR<i32>,
    education_edition_offer: VAR<i32>,
    education_features: bool,
    education_product_id: String,
    rain_level: LE<f32>,
    lightning_level: LE<f32>,
    platform_locked_content: bool,
    multiplayer_intended: bool,
    lan_broadcasting_intended: bool,
    broadcasting_settings_xbox_live: VAR<i32>,
    broadcasting_settings_platform: VAR<i32>,
    commands_enabled: bool,
    texture_pack_required: bool,
    #[len_type(VAR::<u32>)]
    rules: Vec<GameRule>,
    experiments: Experiments,
    bonus_chest: bool,
    start_with_map: bool,
    player_permission: VAR<i32>,
    locked_behavior_packs: bool,
    locked_resource_packs: bool,
    from_locked_template: bool,
    msa_gamertags_only: bool,
    from_template: bool,
    is_template_locked_settings: bool,
    only_spawn_v1_villagers: bool,
    persona_disabled: bool,
    custom_skins_disabled: bool,
    emote_chat_muted: bool,
    base_game_version: BaseGameVersion,
    limited_world_width: LE<i32>,
    limited_world_depth: LE<i32>,
    nether_type: bool,
    edu_shared_uri_resource: EduSharedResourceUri,
    force_experimental_gameplay: bool,
    chat_restriction_level: ChatRestrictionLevel,
    disable_player_interactions: bool,
    server_id: String,
    world_id: String,
    scenario_id: String,
}
