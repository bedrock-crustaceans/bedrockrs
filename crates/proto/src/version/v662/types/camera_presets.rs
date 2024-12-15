use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::CameraPreset;

#[derive(ProtoCodec)]
pub struct CameraPresets {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub presets: Vec<CameraPreset>
}