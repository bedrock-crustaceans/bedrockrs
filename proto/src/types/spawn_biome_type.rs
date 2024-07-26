use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::LE;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum SpawnBiomeType {
    Default,
    UserDefined,
}

impl ProtoCodec for SpawnBiomeType {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            SpawnBiomeType::Default => 0x00,
            SpawnBiomeType::UserDefined => 0x01,
        };

        LE::<i16>::new(int)
            .write(stream)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
