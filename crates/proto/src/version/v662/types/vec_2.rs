use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct Vec2 {
    #[endianness(le)]
    pub x: f32,
    #[endianness(le)]
    pub y: f32,
}