use bedrockrs_core::int::LE;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum ChatRestrictionLevel {
    None = 0x00,
    Dropped = 0x01,
    Disabled = 0x02,
}
