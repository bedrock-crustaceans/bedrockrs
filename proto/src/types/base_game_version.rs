use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;
use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);
