use std::sync::Arc;
use bedrock_core::{Difficulty, LE, VAR};
use bedrock_core::gamemode::Gamemode;
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Difficulty {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        let int = match self {
            Difficulty::Peaceful => { 0x00 }
            Difficulty::Easy => { 0x01 }
            Difficulty::Normal => { 0x02 }
            Difficulty::Hard => { 0x03 }
        };

        match VAR::<i32>::new(int).write(stream) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(ProtoCodecError::IOError(Arc::new(e))) }
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}