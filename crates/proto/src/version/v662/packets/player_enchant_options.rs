use crate::version::v662::types::ItemEnchants;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
pub struct OptionsEntry {
    #[endianness(var)]
    pub cost: u32,
    pub enchants: ItemEnchants,
    pub enchant_name: String,
    #[endianness(var)]
    pub enchant_net_id: u32,
}

#[gamepacket(id = 146)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerEnchantOptionsPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub options: Vec<OptionsEntry>,
}