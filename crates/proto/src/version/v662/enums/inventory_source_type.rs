use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum InventorySourceType {
    InvalidInventory = i32::MAX as isize,
    ContainerInventory = 0,
    GlobalInventory = 1,
    WorldInteraction = 2,
    CreativeInventory = 3,
    NonImplementedFeatureTODO = 99999,
}