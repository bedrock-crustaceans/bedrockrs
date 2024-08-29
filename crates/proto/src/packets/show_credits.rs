use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use crate::types::credits_state::CreditsState;

#[gamepacket(id = 75)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowCreditsPacket {
    actor_id: ActorRuntimeID,
    credits_state: CreditsState,
}
