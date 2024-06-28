use proto_core::ProtoCodec;
use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);
