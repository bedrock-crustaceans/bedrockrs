use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ScoreboardIdentityPacketType {
    Update = 0,
    Remove = 1,
}