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
        VAR::<i32>::new(match self {
            PlayerMovementMode::Client => 0x00,
            PlayerMovementMode::Server => 0x01,
            PlayerMovementMode::ServerWithRewind => 0x02,
        })
        .write(stream)
        .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
