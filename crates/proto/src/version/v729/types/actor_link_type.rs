use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum ActorLinkType {
    None = 0,
    Riding = 1,
    Passenger = 2,
}
