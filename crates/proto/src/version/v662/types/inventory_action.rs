use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{InventorySource, NetworkItemStackDescriptor};

#[derive(ProtoCodec)]
pub struct InventoryAction {
    pub source: InventorySource,
    #[endianness(le)]
    pub slot: u32,
    pub from_item_descriptor: NetworkItemStackDescriptor,
    pub to_item_descriptor: NetworkItemStackDescriptor,
}
