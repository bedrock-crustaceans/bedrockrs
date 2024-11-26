use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ContainerID;

#[derive(ProtoCodec)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum InventorySourceType {
    InvalidInventory = u32::MAX,
    ContainerInventory(ContainerID) = 0,
    GlobalInventory = 1,
    #[endianness(var)]
    WorldInteraction(u32) = 2,
    CreativeInventory = 3,
    NonImplementedFeatureTODO = 99999,
}