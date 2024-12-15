use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct PositionTrackingId {
    #[endianness(var)]
    pub value: i32,
}