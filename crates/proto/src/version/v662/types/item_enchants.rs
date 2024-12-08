use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::Enchant;

#[derive(ProtoCodec)]
struct ItemEnchant {
    pub enchant_type: Enchant::Type,
    pub enchant_level: i8,
}

#[derive(ProtoCodec)]
pub struct ItemEnchants {
    #[endianness(le)]
    pub slot: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub enchants_for_given_activation: Vec<ItemEnchant>
}