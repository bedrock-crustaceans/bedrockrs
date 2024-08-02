use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum Dimension {
    Overworld = 0,
    Nether = 1,
    End = 2,
}
