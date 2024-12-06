use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PredictionType {
    Player = 0,
    Vehicle = 1
}