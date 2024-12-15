use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ItemEnchants;

#[derive(ProtoCodec)]
struct OptionsEntry {
    #[endianness(var)]
    pub cost: u32,
    pub enchants: ItemEnchants,
    pub enchant_name: String,
    #[endianness(var)]
    pub enchant_net_id: u32,
}

#[gamepacket(id = 146)]
#[derive(ProtoCodec)]
pub struct PlayerEnchantOptionsPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub options: Vec<OptionsEntry>,
}