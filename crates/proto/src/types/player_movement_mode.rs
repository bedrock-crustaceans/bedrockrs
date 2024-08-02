use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum PlayerMovementMode {
    Client = 0,
    Server = 1,
    ServerWithRewind = 2,
}
