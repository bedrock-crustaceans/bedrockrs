use bedrockrs_macros::{gamepacket, gamepackets, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use crate::version::v662::enums::PlayerPositionModeComponent;
use crate::version::v662::types::{Vec2, Vec3};

#[gamepacket(id = 19)]
#[derive(ProtoCodec)]
pub struct MovePlayerPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub position: Vec3,
    pub rotation: Vec2,
    #[endianness(le)]
    pub y_head_rotation: f32,
    pub position_mode: PlayerPositionModeComponent::PositionMode,
    pub on_ground: bool,
    pub riding_runtime_id: ActorRuntimeID,
    #[endianness(var)]
    pub tick: u64,
}

// TODO: custom proto impl because of enum variant serialization order