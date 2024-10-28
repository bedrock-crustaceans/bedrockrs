use crate::version::v729::types::interact_action::InteractAction;
use bedrockrs_core::Vec3;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use std::io::Cursor;

#[gamepacket(id = 33)]
#[derive(Debug, Clone)]
pub struct InteractPacket {
    pub action: InteractAction,
    pub target_runtime_id: ActorRuntimeID,
}

impl ProtoCodec for InteractPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let action = match self.action {
            InteractAction::Invalid => 0,
            InteractAction::StopRiding(_) => 3,
            InteractAction::InteractUpdate(_) => 4,
            InteractAction::NpcOpen => 5,
            InteractAction::OpenInventory => 6,
        };

        u8::proto_serialize(&action, stream)?;

        if let InteractAction::InteractUpdate(pos) | InteractAction::StopRiding(pos) = self.action {
            ProtoCodecLE::proto_serialize(&pos, stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let action = u8::proto_deserialize(stream)?;

        let target_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;

        let action = match action {
            0 => InteractAction::Invalid,
            3 => {
                let pos = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;

                InteractAction::StopRiding(pos)
            }
            4 => {
                let pos = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;

                InteractAction::InteractUpdate(pos)
            }
            5 => InteractAction::NpcOpen,
            6 => InteractAction::OpenInventory,
            other => {
                return Err(ProtoCodecError::InvalidEnumID(
                    format!("{other:?}"),
                    String::from("InteractAction"),
                ))
            }
        };

        Ok(Self {
            action,
            target_runtime_id,
        })
    }

    fn get_size_prediction(&self) -> usize {
        todo!()
    }
}
