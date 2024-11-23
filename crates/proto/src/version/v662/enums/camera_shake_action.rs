use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CameraShakeAction {
    Add = 0,
    Stop = 1,
}
