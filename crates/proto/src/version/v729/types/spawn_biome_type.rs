use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i16)]
#[enum_endianness(le)]
pub enum SpawnBiomeType {
    Default = 0,
    UserDefined = 1,
}
