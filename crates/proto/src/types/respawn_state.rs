use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum RespawnState {
    SearchingForSpawn = 0,
    ReadyToSpawn = 1,
    ClientReadyToSpawn = 2,
}
