use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{InventoryLayout, InventoryLeftTabIndex, InventoryRightTabIndex};

#[gamepacket(id = 307)]
#[derive(ProtoCodec)]
pub struct SetPlayerInventoryOptionsPacket {
    pub left_inventory_tab: InventoryLeftTabIndex,
    pub right_inventory_tab: InventoryRightTabIndex,
    pub filtering: bool,
    pub layout_inv: InventoryLayout,
    pub layout_craft: InventoryLayout,
}