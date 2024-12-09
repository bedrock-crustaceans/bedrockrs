use crate::version::v662::types::ItemStackNetIdVariant;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum NetworkItemStackDescriptor {
    Invalid {
        #[endianness(var)]
        id: i32,
    } = 0,
    Valid {
        #[endianness(var)]
        id: i32,
        #[endianness(le)]
        stack_size: u16,
        #[endianness(var)]
        aux_value: u32,
        net_id_variant: Option<ItemStackNetIdVariant>,
        #[endianness(var)]
        block_runtime_id: i32,
        user_data_buffer: String
    } = 1
}