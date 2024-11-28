use bedrockrs_macros::ProtoCodec;
use crate::version::v662::types::InventoryAction;

#[derive(ProtoCodec)]
pub struct InventoryTransaction {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub action: Vec<InventoryAction>,
}