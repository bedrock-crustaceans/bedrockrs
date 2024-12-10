use crate::version::v662::enums::ContainerID;
use crate::version::v662::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventorySlotPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub slot: u32,
    pub item: NetworkItemStackDescriptor,
}