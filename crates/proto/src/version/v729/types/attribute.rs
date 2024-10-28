use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    #[endianness(le)]
    pub min: f32,
    #[endianness(le)]
    pub current: f32,
    #[endianness(le)]
    pub max: f32,
}
