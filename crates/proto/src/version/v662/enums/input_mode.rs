use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum InputMode {
    Undefined = 0,
    Mouse = 1,
    Touch = 2,
    GamePad = 3,
    MotionController = 4,
    Count = 5,
}
