use crate::types::animate_action::AnimateAction;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use varint_rs::{VarintReader, VarintWriter};

#[gamepacket(id = 44)]
#[derive(Debug, Clone)]
pub struct AnimatePlayerPacket {
    pub action: AnimateAction,
    pub target_runtime_id: ActorRuntimeID,
}

impl ProtoCodec for AnimatePlayerPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let action = match self.action {
            AnimateAction::NoAction => 0,
            AnimateAction::Swing { .. } => 1,
            AnimateAction::WakeUp => 3,
            AnimateAction::CriticalHit => 4,
            AnimateAction::MagicCriticalHit => 5,
            AnimateAction::RowRight => 128,
            AnimateAction::RowLeft => 129,
        };

        stream.write_i32_varint(action)?;
        self.target_runtime_id.proto_serialize(stream)?;

        if let AnimateAction::Swing { rowing_time } = self.action {
            stream.write_f32::<LittleEndian>(rowing_time)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let action = stream.read_i32_varint()?;

        let target_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;

        let action = match action {
            0 => AnimateAction::NoAction,
            1 => AnimateAction::Swing {
                rowing_time: stream.read_f32::<LittleEndian>()?,
            },
            3 => AnimateAction::WakeUp,
            4 => AnimateAction::CriticalHit,
            5 => AnimateAction::MagicCriticalHit,
            128 => AnimateAction::RowRight,
            129 => AnimateAction::RowLeft,
            other => {
                return Err(ProtoCodecError::InvalidEnumID(
                    format!("{other:?}"),
                    String::from("AnimateAction"),
                ))
            }
        };

        Ok(Self {
            action,
            target_runtime_id,
        })
    }
}
