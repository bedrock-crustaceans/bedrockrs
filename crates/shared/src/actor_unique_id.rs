use std::io::Cursor;

use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct ActorUniqueID(pub i64);

// ProtoCodec
impl ProtoCodec for ActorUniqueID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        VAR::new(self.0).proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(VAR::proto_deserialize(stream)?.into_inner()))
    }
}
