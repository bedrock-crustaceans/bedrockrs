use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum Mirror {
    None = 0,
    X = 1,
    Z = 2,
    XZ = 3,
}