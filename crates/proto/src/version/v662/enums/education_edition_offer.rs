use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum EducationEditionOffer {
    None = 0,
    RestOfWorld = 1,
    #[deprecated] China = 2,
}