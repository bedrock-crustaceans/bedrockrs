use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::ItemStackResponseSlotInfo;

#[derive(ProtoCodec)]
pub struct ItemStackResponseContainerInfo {
    pub container_net_id: i8,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackResponseSlotInfo>
}