use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CraftingType {
    Inventory = 0,
    Crafting = 1,
}
