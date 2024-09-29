use bedrockrs_core::int::VAR;
use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::types::item_stack_descriptor::ItemStackDescriptor;

#[gamepacket(id = 31)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct MobEquipmentPacket {
    pub runtime_id: VAR<i32>,
    pub item_stack_descriptor: ItemStackDescriptor,
    pub slot: u8,
    pub selected_slot: u8,
    pub container: u8,
}
