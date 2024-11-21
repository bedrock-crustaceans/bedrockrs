pub mod LegacyTelemetryEventPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum AgentResult {
        ActionFail = 0,
        ActionSuccess = 1,
        QueryResultFalse = 2,
        QueryResultTrue = 3,
    }
}