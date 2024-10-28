use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

#[gamepacket(id = 155)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct DebugInfoPacket {
    pub actor_id: ActorUniqueID,
    pub data: String,
}
