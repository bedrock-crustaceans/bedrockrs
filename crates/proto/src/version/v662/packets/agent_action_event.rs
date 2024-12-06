use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::AgentActionType;

#[gamepacket(id = 181)]
#[derive(ProtoCodec)]
pub struct AgentActionEventPacket {
    pub request_id: String,
    pub action_type: AgentActionType,
    pub response: String,
}