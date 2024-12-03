use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkItemStackDescriptor;

#[gamepacket(id = 49)]
#[derive(ProtoCodec)]
pub struct InventoryContentPacket {
    #[endianness(var)]
    pub inventory_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<NetworkItemStackDescriptor>
}