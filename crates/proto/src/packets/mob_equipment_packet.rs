use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::network_item_stack_descriptor::NetworkItemStackDescriptor;

#[derive(ProtoCodec, Debug, Clone)]
pub struct MobEquipmentPacket {
    pub runtime_id: VAR<i32>,
    pub item_stack_descriptor: NetworkItemStackDescriptor,
    pub slot: u8,
    pub selected_slot: u8,
    pub container: u8,
}
