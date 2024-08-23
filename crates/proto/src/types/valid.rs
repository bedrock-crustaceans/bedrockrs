use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

use super::item_stack_net_id_variant::ItemStackNetIdVariant;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Valid {
    pub id: VAR<i32>,
    pub stack_size: LE<u16>,
    pub aux_value: LE<u16>,
    pub include_net_id: bool,
    pub include_net_id_data: Option<ItemStackNetIdVariant>,
    pub block_runtime_id: VAR<i32>,
    pub user_data_buffer: String,
}
