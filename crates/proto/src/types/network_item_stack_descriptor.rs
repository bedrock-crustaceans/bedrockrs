use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_core::ProtoCodec;

use super::item_stack_net_id_variant::ItemStackNetIdVariant;

#[derive(Debug, Clone)]
pub enum NetworkItemStackDescriptor {
    Invalid {
        id: VAR<i32>,
    },
    Valid {
        id: VAR<i32>,
        stack_size: LE<u16>,
        aux_value: LE<u16>,
        include_net_id: bool,
        include_net_id_data: Option<ItemStackNetIdVariant>,
        block_runtime_id: VAR<i32>,
        user_data_buffer: String,
    },
}
impl ProtoCodec for NetworkItemStackDescriptor {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        match self {
            NetworkItemStackDescriptor::Invalid { id } => id.proto_serialize(stream)?,
            NetworkItemStackDescriptor::Valid {
                id,
                stack_size,
                aux_value,
                include_net_id,
                include_net_id_data,
                block_runtime_id,
                user_data_buffer,
            } => {
                id.proto_serialize(stream)?;
                stack_size.proto_serialize(stream)?;
                aux_value.proto_serialize(stream)?;
                include_net_id.proto_serialize(stream)?;
                include_net_id_data.proto_serialize(stream)?;
                block_runtime_id.proto_serialize(stream)?;
                user_data_buffer.proto_serialize(stream)?;
            }
        };

        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        let id = VAR::<i32>::proto_deserialize(stream)?;
        if id.into_inner() == 0 {
            Ok(Self::Invalid { id })
        } else {
            let stack_size = LE::<u16>::proto_deserialize(stream)?;
            let aux_value = LE::<u16>::proto_deserialize(stream)?;
            let include_net_id = bool::proto_deserialize(stream)?;

            let include_net_id_data = if include_net_id {
                Some(ItemStackNetIdVariant::proto_deserialize(stream)?)
            } else {
                None
            };

            Ok(Self::Valid {
                id,
                stack_size,
                aux_value,
                include_net_id,
                include_net_id_data,
                block_runtime_id: VAR::<i32>::proto_deserialize(stream)?,
                user_data_buffer: String::proto_deserialize(stream)?,
            })
        }
    }
}
