use bedrock_core::{Dimension, LE};
use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SpawnSettings {
    // What type is this, I assume biome type... docs just say "type"
    biome_type: LE<u16>,
    user_defined_biome_name: String,
    dimension: Dimension,
}