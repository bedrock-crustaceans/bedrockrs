use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
// TODO: Make sure that it is an u32 and not an i32
#[enum_repr(u32)]
#[enum_endianness(var)]
pub enum InteractionModel {
    Touch = 0,
    Crosshair = 1,
    Classic = 2,
}
