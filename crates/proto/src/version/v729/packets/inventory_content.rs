use crate::version::v729::types::item_stack_descriptor::ItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 49)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct InventoryContentPacket {
    #[endianness(var)]
    pub inventory_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackDescriptor>,
    // TODO: Add FullContainerName
    // TODO: Add DynamicContainerSize
}
