use std::io::Cursor;
use std::sync::Arc;


use bedrockrs_core::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
}

impl ProtoCodec for PlayerMovementMode {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            PlayerMovementMode::Client => 0x00,
            PlayerMovementMode::Server => 0x01,
            PlayerMovementMode::ServerWithRewind => 0x02,
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
