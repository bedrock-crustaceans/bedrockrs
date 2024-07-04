use bedrock_core::Dimension;
use proto_derive::ProtoCodec;

use crate::types::spawn_biome_type::SpawnBiomeType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SpawnSettings {
    biome_type: SpawnBiomeType,
    user_defined_biome_name: String,
    dimension: Dimension,
}