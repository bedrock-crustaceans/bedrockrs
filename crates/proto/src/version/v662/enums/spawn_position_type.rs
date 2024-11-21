use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum SpawnPositionType {
    PlayerRespawn = 0,
    WorldSpawn = 1,
}