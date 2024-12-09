use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ContainerEnumName;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub container_net_id: ContainerEnumName,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}