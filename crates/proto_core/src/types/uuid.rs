use std::io::Cursor;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use uuid::Uuid;
use varint_rs::{VarintReader, VarintWriter};

impl ProtoCodec for Uuid {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let (upper, lower) = self.as_u64_pair();

        stream.write_u64_varint(upper)?;
        stream.write_u64_varint(lower)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Uuid::from_u64_pair(
            stream.read_u64_varint()?,
            stream.read_u64_varint()?,
        ))
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u64>() + size_of::<u64>()
    }
}
