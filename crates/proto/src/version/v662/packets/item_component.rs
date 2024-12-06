use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CompoundTag;

#[derive(ProtoCodec)]
struct ItemsEntry {
    pub component_item_name: String,
    pub component_data: CompoundTag,
}

#[gamepacket(id = 162)]
#[derive(ProtoCodec)]
pub struct ItemComponentPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub items: Vec<ItemsEntry>,
}