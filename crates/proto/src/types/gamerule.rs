use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;
use std::io::Cursor;
use std::sync::Arc;

#[derive(ProtoCodec, Debug, Clone)]
pub struct GameRule {
    name: String,
    editable: bool,
    value: GameRuleValue,
}

#[derive(Debug, Clone)]
pub enum GameRuleValue {
    ValBool(bool),
    ValVarU32(u32),
    ValF32(f32),
}

impl ProtoCodec for GameRuleValue {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = VAR::<i32>::new(match self {
            GameRuleValue::ValBool(v) => {
                v.proto_serialize(stream)?;
                1
            }
            GameRuleValue::ValVarU32(v) => {
                VAR::<u32>::new(*v)
                    .write(stream)
                    .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?;
                2
            }
            GameRuleValue::ValF32(v) => {
                LE::<f32>::new(*v)
                    .write(stream)
                    .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?;
                3
            }
        });

        int.write(stream)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(
            match VAR::<i32>::read(stream)
                .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?
                .into_inner()
            {
                1 => GameRuleValue::ValBool(bool::proto_deserialize(stream)?),
                2 => GameRuleValue::ValVarU32(VAR::<u32>::proto_deserialize(stream)?.into_inner()),
                3 => GameRuleValue::ValF32(LE::<f32>::proto_deserialize(stream)?.into_inner()),
                other => {
                    return Err(ProtoCodecError::InvalidEnumID(
                        format!("{other:?}"),
                        String::from("GameRuleValue"),
                    ));
                }
            },
        )
    }
}
