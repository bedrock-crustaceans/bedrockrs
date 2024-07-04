use bedrock_core::Dimension;
use proto_derive::ProtoCodec;

use crate::types::spawn_biome_type::SpawnBiomeType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SpawnSettings {
    pub biome_type: SpawnBiomeType,
    pub user_defined_biome_name: String,
    pub dimension: Dimension,
}
