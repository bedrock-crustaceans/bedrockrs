use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
struct ItemsEntry {
    pub component_item_name: String,
    #[nbt]
    pub component_data: nbtx::Value, // TODO: NBT Structure
}

#[gamepacket(id = 162)]
#[derive(ProtoCodec)]
pub struct ItemComponentPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub items: Vec<ItemsEntry>,
}