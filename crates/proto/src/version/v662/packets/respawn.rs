use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PlayerRespawnState;
use crate::version::v662::types::{ActorRuntimeID, Vec3};

#[gamepacket(id = 45)]
#[derive(ProtoCodec)]
pub struct RespawnPacket {
    pub position: Vec3,
    pub state: PlayerRespawnState,
    pub player_runtime_id: ActorRuntimeID,
}