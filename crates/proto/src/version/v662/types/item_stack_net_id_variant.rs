use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackNetIdVariant {
    #[endianness(var)]
    pub raw_id: i32
}