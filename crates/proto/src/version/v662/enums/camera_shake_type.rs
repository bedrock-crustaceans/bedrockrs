use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CameraShakeType {
    Positional = 0,
    Rotational = 1,
}