use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
// TODO: Make sure that it is an u32 and not an i32
#[enum_repr(u32)]
#[enum_endianness(var)]
pub enum InputMode {
    Undefined = 0,
    Mouse = 1,
    Touch = 2,
    GamePad = 3,
    MotionController = 4,
}
