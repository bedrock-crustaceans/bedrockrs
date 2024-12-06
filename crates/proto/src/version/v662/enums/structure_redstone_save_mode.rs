use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
pub enum StructureRedstoneSaveMode {
    SavesToMemory = 0,
    SavesToDisk = 1,
}