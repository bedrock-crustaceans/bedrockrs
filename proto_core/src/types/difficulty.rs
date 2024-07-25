use std::io::Cursor;
use std::sync::Arc;


use bedrockrs_core::{Difficulty, VAR};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Difficulty {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            Difficulty::Peaceful => 0x00,
            Difficulty::Easy => 0x01,
            Difficulty::Normal => 0x02,
            Difficulty::Hard => 0x03,
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
