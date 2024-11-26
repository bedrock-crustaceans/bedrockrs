use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct ItemStackNetIdVariant {
    #[endianness(var)]
    pub raw_id: i32
}