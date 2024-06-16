use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);

impl ProtoCodec for BaseGameVersion {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        self.0.proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match String::proto_deserialize(stream) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }
}
