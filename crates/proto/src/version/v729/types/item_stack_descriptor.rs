use super::item_stack_id_variant::ItemStackIdVariant;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[repr(u8)]
#[enum_repr(u8)]
pub enum ItemStackDescriptor {
    Invalid {
        #[endianness(var)]
        id: i32,
    } = 0,
    Valid {
        #[endianness(var)]
        id: i32,
        #[endianness(le)]
        stack_size: u16,
        #[endianness(le)]
        aux_value: u16,
        include_net_id_data: Option<ItemStackIdVariant>,
        #[endianness(var)]
        block_runtime_id: i32,
        user_data_buffer: String,
    } = 1,
}
