use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::version::v729::types::item_stack_descriptor::ItemStackDescriptor;

#[gamepacket(id = 31)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct MobEquipmentPacket {
    #[endianness(var)]
    pub runtime_id: i32,
    pub item_stack_descriptor: ItemStackDescriptor,
    pub slot: u8,
    pub selected_slot: u8,
    pub container: u8,
}
