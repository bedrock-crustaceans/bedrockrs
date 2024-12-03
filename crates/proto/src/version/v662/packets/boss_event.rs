use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::BossEventUpdateType;
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 74)]
#[derive(ProtoCodec)]
pub struct BossEventPacket {
    pub target_actor_id: ActorUniqueID,
    pub event_type: BossEventUpdateType
}