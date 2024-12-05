use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 155)]
#[derive(ProtoCodec)]
pub struct DebugInfoPacket {
    pub actor_id: ActorUniqueID,
    pub data: String,
}