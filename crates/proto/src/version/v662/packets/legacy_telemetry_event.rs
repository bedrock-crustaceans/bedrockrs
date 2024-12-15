use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;
use crate::version::v662::enums::{ActorDamageCause, ActorType, MinecraftEventing};

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
enum AgentResult {
    ActionFail = 0,
    ActionSuccess = 1,
    QueryResultFalse = 2,
    QueryResultTrue = 3,
}

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
enum Type {
    Achievement {
        #[endianness(var)]
        achievement_id: i32,
    } = 0,
    Interaction {
        interaction_type: MinecraftEventing::InteractionType,
        interaction_actor_type: ActorType,
        #[endianness(var)]
        interaction_actor_variant: i32,
        interaction_actor_color: i8,
    } = 1,
    PortalCreated {
        #[endianness(var)]
        dimension_id: i32,
    } = 2,
    PortalUsed {
        #[endianness(var)]
        source_dimension_id: i32,
        #[endianness(var)]
        target_dimension_id: i32,
    } = 3,
    MobKilled {
        #[endianness(var)]
        instigator_actor_id: i64,
        #[endianness(var)]
        target_actor_id: i64,
        instigator_child_actor_type: ActorType,
        damage_source: ActorDamageCause,
        #[endianness(var)]
        trade_tier: i32,
        trader_name: String,
    } = 4,
    CauldronUsed {
        #[endianness(var)]
        contents_color: u32,
        #[endianness(var)]
        contents_type: i32,
        #[endianness(var)]
        fill_level: i32,
    } = 5,
    PlayerDied {
        #[endianness(var)]
        instigator_actor_id: i32,
        #[endianness(var)]
        instigator_mob_variant: i32,
        damage_source: ActorDamageCause,
        died_in_raid: bool,
    } = 6,
    BossKilled {
        #[endianness(var)]
        boss_actor_id: i64,
        #[endianness(var)]
        party_size: i32,
        boss_type: ActorType,
    } = 7,
    AgentCommandObsolete {
        result: AgentResult,
        #[endianness(var)]
        result_number: i32,
        command_name: String,
        result_key: String,
        result_string: String,
    } = 8,
    AgentCreated = 9,
    PatternRemovedObsolete = 10,
    SlashCommand {
        #[endianness(var)]
        success_count: i32,
        #[endianness(var)]
        error_count: i32,
        command_name: String,
        error_list: String,
    } = 11,
    #[deprecated] FishBucketed = 12,
    MobBorn {
        #[endianness(var)]
        baby_entity_type: i32,
        #[endianness(var)]
        baby_entity_variant: i32,
        baby_color: i8,
    } = 13,
    PetDiedObsolete = 14,
    POICauldronUsed {
        block_interaction_type: MinecraftEventing::POIBlockInteractionType,
        #[endianness(var)]
        item_id: i32,
    } = 15,
    ComposterUsed {
        block_interaction_type: MinecraftEventing::POIBlockInteractionType,
        #[endianness(var)]
        item_id: i32,
    } = 16,
    BellUsed {
        #[endianness(var)]
        item_id: i32,
    } = 17,
    ActorDefinition {
        event_name: String,
    } = 18,
    RaidUpdate {
        #[endianness(var)]
        current_raid_wave: i32,
        #[endianness(var)]
        total_raid_waves: i32,
        raid_won: bool,
    } = 19,
    PlayerMovementAnomalyObsolete = 20,
    PlayerMovementCorrectedObsolete = 21,
    HoneyHarvested = 22,
    TargetBlockHit {
        #[endianness(var)]
        redstone_level: i32,
    } = 23,
    PiglinBarter {
        #[endianness(var)]
        item_id: i32,
        bartering_with_player: bool,
    } = 24,
    PlayerWaxedOrUnwaxedCopper {
        #[endianness(var)]
        block_id: i32,
    } = 25,
    CodeBuilderRuntimeAction {
        runtime_action: String,
    } = 26,
    CodeBuilderScoreboard {
        objective_name: String,
        #[endianness(var)]
        score: i32,
    } = 27,
    StriderRiddenInLavaInOverworld = 28,
    SneakCloseToSculkSensor = 29,
    CarefulRestoration = 30,
}

#[gamepacket(id = 65)]
#[derive(ProtoCodec)]
pub struct LegacyTelemetryEventPacket {
    pub target_actor_id: ActorUniqueID,
    pub event_type: Type,
    pub use_player_id: i8,
}

// TODO: custom proto impl, enum variant serialization