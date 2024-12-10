use crate::version::v662::enums::EnchantType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
struct ItemEnchant {
    pub enchant_type: EnchantType,
    pub enchant_level: i8,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemEnchants {
    #[endianness(le)]
    pub slot: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub enchants_for_given_activation: Vec<ItemEnchant>
}