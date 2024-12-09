use crate::version::v662::enums::InventorySourceType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct InventorySource {
    pub source_type: InventorySourceType,
}