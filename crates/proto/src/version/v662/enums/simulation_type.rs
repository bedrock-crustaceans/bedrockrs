use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum SimulationType {
    Game = 0,
    Editor = 1,
    Test = 2,
    Invalid = 3,
}