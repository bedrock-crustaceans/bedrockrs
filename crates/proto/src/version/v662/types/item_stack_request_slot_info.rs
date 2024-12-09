use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub container_net_id: i8,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}