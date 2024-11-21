use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum Mirror {
    None = 0,
    X = 1,
    Z = 2,
    XZ = 3,
}