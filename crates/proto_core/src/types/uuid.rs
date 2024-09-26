use std::io::Cursor;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use uuid::Uuid;
use varint_rs::{VarintReader, VarintWriter};

impl ProtoCodec for Uuid {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let pair = self.as_u64_pair();

        stream.write_u64_varint(pair.0)?;
        stream.write_u64_varint(pair.1)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Uuid::from_u64_pair(
            stream.read_u64_varint()?,
            stream.read_u64_varint()?,
        ))
    }
}
