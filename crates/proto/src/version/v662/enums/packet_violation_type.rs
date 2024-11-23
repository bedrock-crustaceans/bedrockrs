use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PacketViolationType {
    Unknown = -1,
    PacketMalformed = 0,
}