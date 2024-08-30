use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

#[gamepacket(id = 14)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct RemoveEntityPacket {
    pub actor_id: ActorUniqueID,
}
