use uuid::Uuid;
use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{NetworkItemInstanceDescriptor, RecipeIngredient};

#[derive(ProtoCodec)]
pub struct ShapelessChemistryRecipe {
    pub recipe_id: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub ingredients: Vec<RecipeIngredient>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub results: Vec<NetworkItemInstanceDescriptor>,
    pub id: Uuid,
    pub tag: String,
    #[endianness(var)]
    pub priority: i32,
}