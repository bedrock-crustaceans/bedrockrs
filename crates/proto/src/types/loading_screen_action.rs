use bedrockrs_core::int::VAR;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum LoadingScreenAction {
    Unknown = 0,
    Start = 1,
    End = 2,
}
