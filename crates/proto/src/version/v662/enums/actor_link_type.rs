use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ActorLinkType {
    None = 0,
    Riding = 1,
    Passenger = 2,
}
