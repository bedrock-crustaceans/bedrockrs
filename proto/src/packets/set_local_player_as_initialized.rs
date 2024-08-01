use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SetLocalPlayerAsInitializedPacket {
    player_id: ActorRuntimeID,
}
