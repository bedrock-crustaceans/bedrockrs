use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct NetworkBlockPosition {
    #[endianness(var)]
    x: i32,
    #[endianness(var)]
    y: u32,
    #[endianness(var)]
    z: i32,
}