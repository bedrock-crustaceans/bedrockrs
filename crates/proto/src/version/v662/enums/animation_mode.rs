use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum AnimationMode {
    None = 0,
    Layers = 1,
    Blocks = 2,
}