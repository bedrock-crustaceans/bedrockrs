use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::LE;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum RespawnState {
    SearchingForSpawn = 0,
    ReadyToSpawn = 1,
    ClientReadyToSpawn = 2,
}
