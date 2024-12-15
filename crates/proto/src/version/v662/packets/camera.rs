use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 73)]
#[derive(ProtoCodec)]
pub struct CameraPacket {
    pub camera_id: ActorUniqueID,
    pub target_player_id: ActorUniqueID,
}