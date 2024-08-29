use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

#[derive(ProtoCodec, Debug, Clone)]
pub struct RemoveEntityPacket {
    pub target_actor_id: ActorUniqueID,
}
