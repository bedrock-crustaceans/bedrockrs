use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;

use crate::error::ProtoCodecError;

pub mod error;
pub mod types;

pub trait ProtoCodec: Sized {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>;
}
