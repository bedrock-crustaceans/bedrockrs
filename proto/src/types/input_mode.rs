use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::VAR;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum InputMode {
    Undefined = 0,
    Mouse = 1,
    Touch = 2,
    GamePad = 3,
    MotionController = 4,
}
