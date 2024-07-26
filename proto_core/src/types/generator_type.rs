use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::{GeneratorType, VAR};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for GeneratorType {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            GeneratorType::Legacy => 0x00,
            GeneratorType::Overworld => 0x01,
            GeneratorType::Flat => 0x02,
            GeneratorType::Nether => 0x03,
            GeneratorType::End => 0x04,
            GeneratorType::Void => 0x05,
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
