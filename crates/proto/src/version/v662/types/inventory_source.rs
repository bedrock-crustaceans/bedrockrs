use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::InventorySourceType;

#[derive(ProtoCodec)]
pub struct InventorySource {
    pub source_type: InventorySourceType,
}