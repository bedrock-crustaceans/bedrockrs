use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PacketViolationSeverity {
    Unknown = -1,
    Warning = 0,
    FinalWarning = 1,
    TerminatingConnection = 2,
}