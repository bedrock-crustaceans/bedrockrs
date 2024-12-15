use uuid::Uuid;
use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::{NetworkItemInstanceDescriptor, RecipeIngredient};

#[derive(ProtoCodec)]
pub struct ShapelessRecipe {
    pub recipe_unique_id: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub ingredient_list: Vec<RecipeIngredient>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub production_list: Vec<NetworkItemInstanceDescriptor>,
    pub recipe_id: Uuid,
    pub recipe_tag: String,
    #[endianness(var)]
    pub priority: i32,
}