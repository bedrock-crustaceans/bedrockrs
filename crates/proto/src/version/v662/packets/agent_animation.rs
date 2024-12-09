use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 304)]
#[derive(ProtoCodec)]
pub struct AgentAnimationPacket {
    pub agent_animation: i8,
    pub runtime_id: ActorRuntimeID,
}