use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[gamepacket(id = 113)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub target_actor_id: ActorRuntimeID,
}
