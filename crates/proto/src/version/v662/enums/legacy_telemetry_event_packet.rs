pub mod LegacyTelemetryEventPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum AgentResult {
        ActionFail = 0,
        ActionSuccess = 1,
        QueryResultFalse = 2,
        QueryResultTrue = 3,
    }
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Achievement = 0,
        Interaction = 1,
        PortalCreated = 2,
        PortalUsed = 3,
        MobKilled = 4,
        CauldronUsed = 5,
        PlayerDied = 6,
        BossKilled = 7,
        AgentCommandObsolete = 8,
        AgentCreated = 9,
        PatternRemovedObsolete = 10,
        SlashCommand = 11,
        #[deprecated] FishBucketed = 12,
        MobBorn = 13,
        PetDiedObsolete = 14,
        POICauldronUsed = 15,
        ComposterUsed = 16,
        BellUsed = 17,
        ActorDefinition = 18,
        RaidUpdate = 19,
        PlayerMovementAnomalyObsolete = 20,
        PlayerMovementCorrectedObsolete = 21,
        HoneyHarvested = 22,
        TargetBlockHit = 23,
        PiglinBarter = 24,
        PlayerWaxedOrUnwaxedCopper = 25,
        CodeBuilderRuntimeAction = 26,
        CodeBuilderScoreboard = 27,
        StriderRiddenInLavaInOverworld = 28,
        SneakCloseToSculkSensor = 29,
        CarefulRestoration = 30,
    }
}