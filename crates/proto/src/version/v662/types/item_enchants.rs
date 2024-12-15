use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
struct ItemEnchant {
    pub enchant_type: i8,
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