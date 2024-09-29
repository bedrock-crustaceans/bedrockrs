use bedrockrs_core::int::VAR;
use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::types::item_stack_descriptor::ItemStackDescriptor;

#[gamepacket(id = 49)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct InventoryContentPacket {
    pub inventory_id: VAR<u32>,
    #[len_repr(VAR::<u32>)]
    pub slots: Vec<ItemStackDescriptor>,
}
