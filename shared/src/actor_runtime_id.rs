use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ActorRuntimeID(pub u64);

impl ProtoCodec for ActorRuntimeID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        VAR::new(self.0).proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(VAR::proto_deserialize(stream)?.into_inner()))
    }
}
