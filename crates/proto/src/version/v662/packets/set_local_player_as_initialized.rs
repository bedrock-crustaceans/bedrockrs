use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 113)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub player_id: ActorRuntimeID,
}