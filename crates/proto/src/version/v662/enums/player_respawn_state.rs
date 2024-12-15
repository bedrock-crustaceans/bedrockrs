use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PlayerRespawnState {
    SearchingForSpawn = 0,
    ReadyToSpawn = 1,
    ClientReadyToSpawn = 2,
}