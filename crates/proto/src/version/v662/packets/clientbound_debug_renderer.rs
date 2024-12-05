use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec3;

#[derive(ProtoCodec)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
enum Type {
    Invalid = 0,
    ClearDebugMarkers = 1,
    AddDebugMarkerCube {
        text: String,
        position: Vec3,
        #[endianness(le)]
        r: f32,
        #[endianness(le)]
        g: f32,
        #[endianness(le)]
        b: f32,
        #[endianness(le)]
        a: f32,
        #[endianness(le)]
        millisecond_duration: u64
    } = 2,
}

#[gamepacket(id = 163)]
#[derive(ProtoCodec)]
pub struct ClientboundDebugRendererPacket {
    pub debug_marker_type: Type,
}