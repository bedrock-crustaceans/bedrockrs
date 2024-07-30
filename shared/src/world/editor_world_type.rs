use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum EditorWorldType {
    NotEditor = 0,
    Project = 1,
    TestLevel = 2,
}

