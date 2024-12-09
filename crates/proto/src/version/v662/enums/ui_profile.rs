use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub enum UIProfile {
    Classic = 0,
    Pocket = 1,
    None = 2,
    Count = 3,
}