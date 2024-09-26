use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub enum BossEventType {
    Add {
        name: String,
        health_percentage: LE<f32>,
        darken_screen: LE<u16>,
        color: VAR<u32>,
        overlay: VAR<u32>,
    },
    PlayerAdded {
        actor_id: ActorUniqueID,
    },
    Remove,
    PlayerRemoved {
        actor_id: ActorUniqueID,
    },
    UpdatePercent {
        health_percentage: LE<f32>,
    },
    UpdateName {
        name: String,
    },
    UpdateProperties {
        darken_screen: LE<u16>,
        color: VAR<u32>,
        overlay: VAR<u32>,
    },
    UpdateStyle {
        color: VAR<u32>,
        overlay: VAR<u32>,
    },
    Query {
        actor_id: ActorUniqueID,
    },
}

impl ProtoCodec for BossEventType {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        match self {
            BossEventType::Add {
                name,
                health_percentage,
                darken_screen,
                color,
                overlay,
            } => {
                VAR::new(0).proto_serialize(buf)?;

                name.proto_serialize(buf)?;
                health_percentage.proto_serialize(buf)?;
                darken_screen.proto_serialize(buf)?;
                color.proto_serialize(buf)?;
                overlay.proto_serialize(buf)?;
            }
            BossEventType::PlayerAdded { actor_id } => {
                VAR::new(1).proto_serialize(buf)?;

                actor_id.proto_serialize(buf)?;
            }
            BossEventType::Remove => VAR::new(2).proto_serialize(buf)?,
            BossEventType::PlayerRemoved { actor_id } => {
                VAR::new(3).proto_serialize(buf)?;

                actor_id.proto_serialize(buf)?;
            }
            BossEventType::UpdatePercent { health_percentage } => {
                VAR::new(4).proto_serialize(buf)?;

                health_percentage.proto_serialize(buf)?;
            }
            BossEventType::UpdateName { name } => {
                VAR::new(5).proto_serialize(buf)?;

                name.proto_serialize(buf)?;
            }
            BossEventType::UpdateProperties {
                darken_screen,
                color,
                overlay,
            } => {
                VAR::new(6).proto_serialize(buf)?;

                darken_screen.proto_serialize(buf)?;
                color.proto_serialize(buf)?;
                overlay.proto_serialize(buf)?;
            }
            BossEventType::UpdateStyle { color, overlay } => {
                VAR::new(7).proto_serialize(buf)?;

                color.proto_serialize(buf)?;
                overlay.proto_serialize(buf)?;
            }
            BossEventType::Query { actor_id } => {
                VAR::new(8).proto_serialize(buf)?;

                actor_id.proto_serialize(buf)?;
            }
        };

        Ok(())
    }

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(match VAR::<u32>::proto_deserialize(buf)?.into_inner() {
            0 => {
                let name = String::proto_deserialize(buf)?;
                let health_percentage = LE::<f32>::proto_deserialize(buf)?;
                let darken_screen = LE::<u16>::proto_deserialize(buf)?;
                let color = VAR::<u32>::proto_deserialize(buf)?;
                let overlay = VAR::<u32>::proto_deserialize(buf)?;

                BossEventType::Add {
                    name,
                    health_percentage,
                    darken_screen,
                    color,
                    overlay,
                }
            }
            1 => {
                let actor_id = ActorUniqueID::proto_deserialize(buf)?;

                BossEventType::PlayerAdded { actor_id }
            }
            2 => BossEventType::Remove,
            3 => {
                let actor_id = ActorUniqueID::proto_deserialize(buf)?;

                BossEventType::PlayerRemoved { actor_id }
            }
            4 => {
                let health_percentage = LE::<f32>::proto_deserialize(buf)?;

                BossEventType::UpdatePercent { health_percentage }
            }
            5 => {
                let name = String::proto_deserialize(buf)?;

                BossEventType::UpdateName { name }
            }
            6 => {
                let darken_screen = LE::<u16>::proto_deserialize(buf)?;
                let color = VAR::<u32>::proto_deserialize(buf)?;
                let overlay = VAR::<u32>::proto_deserialize(buf)?;

                BossEventType::UpdateProperties {
                    darken_screen,
                    color,
                    overlay,
                }
            }
            7 => {
                let color = VAR::<u32>::proto_deserialize(buf)?;
                let overlay = VAR::<u32>::proto_deserialize(buf)?;

                BossEventType::UpdateStyle { color, overlay }
            }
            8 => {
                let actor_id = ActorUniqueID::proto_deserialize(buf)?;

                BossEventType::Query { actor_id }
            }
            other => {
                return Err(ProtoCodecError::InvalidEnumID(
                    format!("{other:?}"),
                    String::from("BossEventType"),
                ));
            }
        })
    }
}
