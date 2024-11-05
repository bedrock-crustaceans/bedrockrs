use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CraftingDataEntryType {
    ShapelessRecipe = 0,
    ShapedRecipe = 1,
    FurnaceRecipe = 2,
    FurnaceAuxRecipe = 3,
    MultiRecipe = 4,
    ShulkerBoxRecipe = 5,
    ShapelessChemistryRecipe = 6,
    ShapedChemistryRecipe = 7,
    SmithingTransformRecipe = 8,
    SmithingTrimRecipe = 9,
    Count = 10,
}
