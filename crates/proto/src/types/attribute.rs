use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub min: LE<f32>,
    pub current: LE<f32>,
    pub max: LE<f32>,
}
