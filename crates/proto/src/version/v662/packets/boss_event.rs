use crate::version::v662::enums::BossEventUpdateType;
use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 74)]
#[derive(ProtoCodec)]
pub struct BossEventPacket {
    pub target_actor_id: ActorUniqueID,
    pub event_type: BossEventUpdateType
}