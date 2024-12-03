use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 14)]
#[derive(ProtoCodec)]
pub struct RemoveActorPacket {
    pub target_actor_id: ActorUniqueID
}
