use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum InventoryLayout {
    None = 0,
    Survival = 1,
    RecipeBook = 2,
    Creative = 3,
    Count = 4,
}