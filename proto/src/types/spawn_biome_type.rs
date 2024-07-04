use std::sync::Arc;

use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::LE;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum SpawnBiomeType {
    Default,
    UserDefined,
}

impl ProtoCodec for SpawnBiomeType {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        let int = match self {
            SpawnBiomeType::Default => 0x00,
            SpawnBiomeType::UserDefined => 0x01,
        };

        match LE::<i16>::new(int).write(stream) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
