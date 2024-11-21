use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum StructureRedstoneSaveMode {
    SavesToMemory = 0,
    SavesToDisk = 1,
}