use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::Cursor;
use varint_rs::{VarintReader, VarintWriter};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ItemStackIdVariant(i32);

impl ProtoCodec for ItemStackIdVariant {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        stream.write_i32_varint(self.0)?;
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let id = stream.read_i32_varint()?;
        Ok(Self(id))
    }

    fn get_size_prediction(&self) -> usize {
        self.0.get_size_prediction()
    }
}
