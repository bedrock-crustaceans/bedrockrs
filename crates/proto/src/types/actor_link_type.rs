use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;


#[derive(ProtoCodec,Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum ActorLinkType {
    None = 0,
    Riding = 1,
    Passenger = 2,
}