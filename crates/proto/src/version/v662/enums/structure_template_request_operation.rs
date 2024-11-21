use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum StructureTemplateRequestOperation {
    None = 0,
    ExportFromSaveMode = 1,
    ExportFromLoadMode = 2,
    QuerySavedStructure = 3,
    Import = 4,
}