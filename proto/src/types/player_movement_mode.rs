use std::sync::Arc;
use bedrock_core::LE;
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
}

impl ProtoCodec for PlayerMovementMode {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        let int = match self {
            PlayerMovementMode::Client => { 0x00 }
            PlayerMovementMode::Server => { 0x01 }
            PlayerMovementMode::ServerWithRewind => { 0x02 }
        };

        match LE::<u8>::new(int).write(stream) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(ProtoCodecError::IOError(Arc::new(e))) }
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}