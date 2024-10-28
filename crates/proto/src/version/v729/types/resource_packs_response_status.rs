use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Copy, Clone, Eq, PartialEq)]
#[enum_repr(i8)]
pub enum ResourcePacksResponseStatus {
    None = 0,
    Refused = 1,
    SendPacks = 2,
    HaveAllPacks = 3,
    Completed = 4,
}
