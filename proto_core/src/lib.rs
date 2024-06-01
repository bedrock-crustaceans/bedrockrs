use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;

use crate::error::ProtoCodecError;

pub mod error;
pub mod types;

pub trait ProtoCodec: Sized {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>;
}
