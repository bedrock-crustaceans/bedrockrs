use std::io::Cursor;
use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};

#[derive(Debug, Clone)]
pub enum ItemStackDescriptor {
    Air,
    Stack {
        network_id: i32,
        stack: ItemStack,
    }
}

impl ProtoCodec for ItemStackDescriptor {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        match self {
            ItemStackDescriptor::Air => {
                <i32 as ProtoCodecVAR>::proto_serialize(&0, stream)?;
            }
            ItemStackDescriptor::Stack { network_id, stack} => {
                <i32 as ProtoCodecVAR>::proto_serialize(network_id, stream)?;
                stack.proto_serialize(stream)?;
            }
        }
        
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let network_id = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        
        let stack_descriptor = if network_id == 0 {
            Self::Air
        } else { 
            let stack = ItemStack::proto_deserialize(stream)?;
            Self::Stack { network_id, stack }
        };
        
        Ok(stack_descriptor)
    }

    fn get_size_prediction(&self) -> usize {
        match self {
            ItemStackDescriptor::Air => <i32 as ProtoCodecVAR>::get_size_prediction(&0),
            ItemStackDescriptor::Stack { network_id, stack } => {
                <i32 as ProtoCodecVAR>::get_size_prediction(network_id) + stack.get_size_prediction()
            }
        }
    }
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct ItemStack {
    #[endianness(le)]
    stack_size: u16,
    #[endianness(var)]
    aux: u32,
    net_id: Option<ItemStackIdVariant>,
    #[endianness(var)]
    block_runtime_id: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    user_data_buffer: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ItemStackIdVariant(pub i32);

impl ProtoCodec for ItemStackIdVariant {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        <i32 as ProtoCodecVAR>::proto_serialize(&self.0, stream)?;
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let id = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        Ok(Self(id))
    }

    fn get_size_prediction(&self) -> usize {
        <i32 as ProtoCodecVAR>::get_size_prediction(&self.0)
    }
}
