use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub enum CraftingType {
    Inventory = 0,
    Crafting = 1,
}
