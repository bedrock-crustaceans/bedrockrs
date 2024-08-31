use crate::types::animate_action::AnimateAction;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::gamepacket;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use std::io::Cursor;

#[gamepacket(id = 44)]
#[derive(Debug, Clone)]
pub struct AnimatePlayerPacket {
    action: AnimateAction,
    target_runtime_id: ActorRuntimeID,
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

        VAR::<i32>::new(action).proto_serialize(stream)?;
        self.target_runtime_id.proto_serialize(stream)?;

        if let AnimateAction::Swing { rowing_time } = self.action {
            LE::new(rowing_time).proto_serialize(stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let action = VAR::<i32>::proto_deserialize(stream)?.into_inner();

        let target_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;

        let action = match action {
            0 => AnimateAction::NoAction,
            1 => AnimateAction::Swing {
                rowing_time: LE::<f32>::proto_deserialize(stream)?.into_inner(),
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
