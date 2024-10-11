use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use std::io::Cursor;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);

impl ProtoCodec for BaseGameVersion {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.0.proto_serialize(stream)?;
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let base_game_version = String::proto_deserialize(stream)?;
        Ok(BaseGameVersion(base_game_version))
    }

    fn get_size_prediction(&self) -> usize {
        self.0.get_size_prediction()
    }
}
