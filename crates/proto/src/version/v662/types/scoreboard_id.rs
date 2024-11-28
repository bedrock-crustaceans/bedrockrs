use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct ScoreboardId {
    #[endianness(var)]
    pub id: i64,
}