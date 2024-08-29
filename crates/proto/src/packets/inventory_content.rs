use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::network_item_stack_descriptor::NetworkItemStackDescriptor;

#[derive(ProtoCodec, Debug, Clone)]
pub struct InventoryContentPacket {
    pub inventory_id: VAR<u32>,
    #[len_repr(VAR::<u32>)]
    pub slots: Vec<NetworkItemStackDescriptor>,
}
