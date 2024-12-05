use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum CameraShakeType {
    Positional = 0,
    Rotational = 1,
}