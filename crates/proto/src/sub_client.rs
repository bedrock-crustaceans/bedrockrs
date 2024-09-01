use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::LE;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum SubClientID {
    PrimaryClient = 0,
    Client2 = 1,
    Client3 = 2,
    Client4 = 3,
}
