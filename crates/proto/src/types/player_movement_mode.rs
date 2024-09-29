use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum PlayerMovementMode {
    Client = 0,
    Server = 1,
    ServerWithRewind = 2,
}
