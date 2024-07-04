use bedrock_core::VAR;
use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkBlockPos {
    x: VAR<i32>,
    y: VAR<u32>,
    z: VAR<i32>,
}