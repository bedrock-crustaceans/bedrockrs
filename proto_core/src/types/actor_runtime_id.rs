use bedrock_core::{ActorRuntimeID, VAR};
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for ActorRuntimeID {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        VAR::new(self.0).proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        Ok(Self(VAR::<u64>::proto_deserialize(stream)?.into_inner()))
    }
}
