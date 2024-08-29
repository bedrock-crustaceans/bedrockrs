use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

#[derive(ProtoCodec, Debug, Clone)]
pub struct CameraPacket {
    pub camera_id: ActorUniqueID,
    pub target_player_id: ActorUniqueID,
}
