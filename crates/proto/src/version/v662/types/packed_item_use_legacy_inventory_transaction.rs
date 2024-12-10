use crate::version::v662::enums::ItemUseInventoryTransactionType;
use crate::version::v662::types::{InventoryAction, NetworkBlockPosition, NetworkItemStackDescriptor};
use bedrockrs_core::Vec3;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
struct ContainerSlotEntry {
    pub container_enum_name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<i8>
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PackedItemUseLegacyInventoryTransaction {
    Invalid{
        #[endianness(var)]
        id: i32,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        actions: Vec<InventoryAction>,
        action_type: ItemUseInventoryTransactionType,
        position: NetworkBlockPosition,
        #[endianness(var)]
        face: i32,
        #[endianness(var)]
        slot: i32,
        item: NetworkItemStackDescriptor,
        #[endianness(le)]
        from_position: Vec3<f32>,
        #[endianness(le)]
        click_position: Vec3<f32>,
        #[endianness(var)]
        target_block_id: u32,
    } = 0,
    Valid {
        #[endianness(var)]
        id: i32,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        container_slots: Vec<ContainerSlotEntry>,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        actions: Vec<InventoryAction>,
        action_type: ItemUseInventoryTransactionType,
        position: NetworkBlockPosition,
        #[endianness(var)]
        face: i32,
        #[endianness(var)]
        slot: i32,
        item: NetworkItemStackDescriptor,
        #[endianness(le)]
        from_position: Vec3<f32>,
        #[endianness(le)]
        click_position: Vec3<f32>,
        #[endianness(var)]
        target_block_id: u32,
    } = 1
}