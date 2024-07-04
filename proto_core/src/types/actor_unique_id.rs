use bedrock_core::{ActorRuntimeID, ActorUniqueID, VAR};
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for ActorUniqueID {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        self.0.proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        Ok(Self(VAR::<i64>::proto_deserialize(stream)?))
    }
}