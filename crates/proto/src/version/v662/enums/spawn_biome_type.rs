use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum SpawnBiomeType {
    Default = 0,
    UserDefined = 1,
}
