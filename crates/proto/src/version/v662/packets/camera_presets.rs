use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CameraPresets;

#[gamepacket(id = 198)]
#[derive(ProtoCodec)]
pub struct CameraPresetsPacket {
    pub camera_presets: CameraPresets,
}