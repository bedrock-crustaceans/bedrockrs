use bedrock_core::VAR;
use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkBlockPos {
    pub x: VAR<i32>,
    pub y: VAR<u32>,
    pub z: VAR<i32>,
}