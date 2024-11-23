use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum StructureTemplateResponseType {
    None = 0,
    Export = 1,
    Query = 2,
    Import = 3,
}