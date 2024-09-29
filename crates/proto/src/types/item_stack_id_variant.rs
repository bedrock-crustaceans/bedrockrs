use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ItemStackIdVariant(#[endianness(var)] i32);
