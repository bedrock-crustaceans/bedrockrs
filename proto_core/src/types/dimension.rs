use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::{Dimension, VAR};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Dimension {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            Dimension::Overworld => 0x00,
            Dimension::Nether => 0x01,
            Dimension::End => 0x02,
        };

        match VAR::<i32>::new(int).write(stream) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
