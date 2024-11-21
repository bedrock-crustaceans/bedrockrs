use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum NewInteractionModel {
    Touch = 0,
    Crosshair = 1,
    Classic = 2,
    Count = 3,
}