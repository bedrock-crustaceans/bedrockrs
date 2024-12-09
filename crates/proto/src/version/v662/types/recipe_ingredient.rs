use crate::version::v662::enums::ItemDescriptor;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct RecipeIngredient {
    pub internal_type: ItemDescriptor::InternalType,
    #[endianness(var)]
    pub stack_size: i32,
}