use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{CameraShakeAction, CameraShakeType};

#[gamepacket(id = 159)]
#[derive(ProtoCodec)]
pub struct CameraShakePacket {
    #[endianness(le)]
    pub intensity: f32,
    #[endianness(le)]
    pub seconds: f32,
    pub shake_type: CameraShakeType,
    pub shake_action: CameraShakeAction,
}