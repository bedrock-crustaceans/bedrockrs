use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum LoadingScreenAction {
    Unknown = 0,
    Start = 1,
    End = 2,
}
