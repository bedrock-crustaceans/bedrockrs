use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
pub enum RespawnState {
    SearchingForSpawn = 0,
    ReadyToSpawn = 1,
    ClientReadyToSpawn = 2,
}
