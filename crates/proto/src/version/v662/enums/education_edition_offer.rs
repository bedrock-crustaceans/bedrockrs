use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum EducationEditionOffer {
    None = 0,
    RestOfWorld = 1,
    ChinaDeprecated = 2,
}