use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum Dimension {
    Overworld = 0,
    Nether = 1,
    End = 2,
    Undefined = 3,
}
