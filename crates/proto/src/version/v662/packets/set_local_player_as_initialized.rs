use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[gamepacket(id = 113)]
#[derive(ProtoCodec)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub player_id: ActorRuntimeID,
}