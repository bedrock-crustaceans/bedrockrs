use crate::version::v662::enums::PlayerRespawnState;
use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 45)]
#[derive(ProtoCodec)]
pub struct RespawnPacket {
    #[endianness(le)]
    pub position: Vec3<f32>,
    pub state: PlayerRespawnState,
    pub player_runtime_id: ActorRuntimeID,
}