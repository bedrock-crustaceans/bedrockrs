use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    SurvivalSpectator = 3,
    CreativeSpectator = 4,
    Default = 5,
    Spectator = 6,
}
