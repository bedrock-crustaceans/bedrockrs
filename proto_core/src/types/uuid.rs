use std::io::Cursor;
use bedrockrs_core::{Uuid, LE};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Uuid {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let pair = self.as_u64_pair();

        LE::<u64>::new(pair.0).proto_serialize(stream)?;
        LE::<u64>::new(pair.1).proto_serialize(stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let first = LE::<u64>::proto_deserialize(stream)?.into_inner();
        let second = LE::<u64>::proto_deserialize(stream)?.into_inner();

        Ok(Uuid::from_u64_pair(first, second))
    }
}
