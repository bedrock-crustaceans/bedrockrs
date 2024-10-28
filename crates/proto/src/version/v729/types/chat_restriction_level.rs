use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
pub enum ChatRestrictionLevel {
    None = 0,
    Dropped = 1,
    Disabled = 2,
}
