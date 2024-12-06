use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ItemDescriptor;

#[derive(ProtoCodec)]
pub struct RecipeIngredient {
    pub internal_type: ItemDescriptor::InternalType,
    #[endianness(var)]
    pub stack_size: i32,
}