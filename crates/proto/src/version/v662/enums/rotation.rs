use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum Rotation {
    None = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    Total = 4,
}