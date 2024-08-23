use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_core::ProtoCodec;

use super::{item_stack_net_id_variant::ItemStackNetIdVariant, valid::Valid};

#[derive(Debug, Clone)]
pub enum NetworkItemStackDescriptor {
    Invalid { id: VAR<i32> },
    Valid { valid: Valid },
}
impl ProtoCodec for NetworkItemStackDescriptor {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
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
                valid: Valid {
                    id,
                    stack_size,
                    aux_value,
                    include_net_id,
                    include_net_id_data,
                    block_runtime_id: VAR::<i32>::proto_deserialize(stream)?,
                    user_data_buffer: String::proto_deserialize(stream)?,
                },
            })
        }
    }
}
