use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum InventoryLeftTabIndex {
    None = 0,
    RecipeConstruction = 1,
    RecipeEquipment = 2,
    RecipeItems = 3,
    RecipeNature = 4,
    RecipeSearch = 5,
    Survival = 6,
    Count = 7,
}