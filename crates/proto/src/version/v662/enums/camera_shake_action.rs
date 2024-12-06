use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum CameraShakeAction {
    Add = 0,
    Stop = 1,
}
