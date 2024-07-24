use bedrockrs_core::read::ByteStreamRead;
use bedrockrs_core::write::ByteStreamWrite;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct GameRule {
    name: String,
    editable: bool,
    value: GameRuleValue,
}

#[derive(Debug, Clone)]
pub enum GameRuleValue {
    ValBool(bool),
    ValI32(i32),
    ValF32(f32),
}

impl ProtoCodec for GameRuleValue {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        todo!()
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
