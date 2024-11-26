use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{NetworkItemInstanceDescriptor, RecipeIngredient};

#[derive(ProtoCodec)]
pub struct SmithingTransformRecipe {
    pub recipe_id: String,
    pub template_ingredient: RecipeIngredient,
    pub base_ingredient: RecipeIngredient,
    pub addition_ingredient: RecipeIngredient,
    pub result: NetworkItemInstanceDescriptor,
    pub tag: String,
}