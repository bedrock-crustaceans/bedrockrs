use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ScorePacketType {
    Change = 0,
    Remove = 1,
}