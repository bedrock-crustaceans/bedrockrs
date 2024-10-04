use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
pub enum ChatRestrictionLevel {
    None = 0x00,
    Dropped = 0x01,
    Disabled = 0x02,
}
