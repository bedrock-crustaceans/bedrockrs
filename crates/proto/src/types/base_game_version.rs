use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);
