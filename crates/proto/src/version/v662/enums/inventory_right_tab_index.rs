use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum InventoryRightTabIndex {
    None = 0,
    FullScreen = 1,
    Crafting = 2,
    Armor = 3,
    Count = 4,
}