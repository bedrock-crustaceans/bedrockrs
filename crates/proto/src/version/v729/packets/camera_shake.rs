use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 159)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct CameraShakePacket {
    #[endianness(le)]
    pub intensity: f32,
    #[endianness(le)]
    pub seconds: f32,
    // TODO: Turn into enum
    pub shake_type: i8,
    // TODO: Turn into enum
    pub shake_action: i8,
}
