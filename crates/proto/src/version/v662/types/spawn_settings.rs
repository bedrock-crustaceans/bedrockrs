use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::SpawnBiomeType;

#[derive(ProtoCodec)]
pub struct SpawnSettings {
    pub spawn_type: SpawnBiomeType,
    pub user_defined_biome_name: String,
    #[endianness(var)]
    pub dimension: i32,
}
