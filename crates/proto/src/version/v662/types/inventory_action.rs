use crate::version::v662::types::{InventorySource, NetworkItemStackDescriptor};
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct InventoryAction {
    pub source: InventorySource,
    #[endianness(le)]
    pub slot: u32,
    pub from_item_descriptor: NetworkItemStackDescriptor,
    pub to_item_descriptor: NetworkItemStackDescriptor,
}
