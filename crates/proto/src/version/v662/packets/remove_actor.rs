use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 14)]
#[derive(ProtoCodec)]
pub struct RemoveActorPacket {
    pub target_actor_id: ActorUniqueID
}
