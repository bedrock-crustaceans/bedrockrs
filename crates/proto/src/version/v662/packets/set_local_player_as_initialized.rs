use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 113)]
#[derive(ProtoCodec)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub player_id: ActorRuntimeID,
}