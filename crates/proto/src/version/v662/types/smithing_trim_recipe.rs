use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::RecipeIngredient;

#[derive(ProtoCodec)]
pub struct SmithingTrimRecipe {
    pub recipe_id: String,
    pub template_ingredient: RecipeIngredient,
    pub base_ingredient: RecipeIngredient,
    pub addition_ingredient: RecipeIngredient,
    pub tag: String,
}