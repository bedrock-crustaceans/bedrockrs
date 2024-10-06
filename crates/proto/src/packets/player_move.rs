use bedrockrs_core::Vec2;
use bedrockrs_core::Vec3;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use std::io::Cursor;

#[gamepacket(id = 19)]
#[derive(Debug, Clone)]
pub struct MovePlayerPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub position: Vec3<f32>,
    pub rotation: Vec2<f32>,
    pub head_rotation: f32,
    pub position_mode: u8,
    pub on_ground: bool,
    pub riding_runtime_id: ActorRuntimeID,
    pub teleportation_cause: Option<i32>,
    pub source_actor_type: Option<i32>,
    pub tick: i64,
}

impl ProtoCodec for MovePlayerPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        unimplemented!();
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let player_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;
        let position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let rotation = <Vec2<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let head_rotation = <f32 as ProtoCodecLE>::proto_deserialize(stream)?;
        let position_mode = u8::proto_deserialize(stream)?;
        let on_ground = bool::proto_deserialize(stream)?;
        let riding_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;

        let mut teleportation_cause: Option<i32> = None;
        let mut source_actor_type: Option<i32> = None;

        // teleportation mode..
        if position_mode == 2 {
            teleportation_cause = Some(<i32 as ProtoCodecLE>::proto_deserialize(stream)?);
            source_actor_type = Some(<i32 as ProtoCodecLE>::proto_deserialize(stream)?);
        }

        let tick = <i64 as ProtoCodecVAR>::proto_deserialize(stream)?;

        Ok(Self {
            player_runtime_id,
            position,
            rotation,
            head_rotation,
            position_mode,
            on_ground,
            riding_runtime_id,
            teleportation_cause,
            source_actor_type,
            tick,
        })
    }
}
