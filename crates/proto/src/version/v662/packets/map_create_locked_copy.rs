use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 131)]
#[derive(ProtoCodec)]
pub struct MapCreateLockedCopyPacket {
    pub original_map_id: ActorUniqueID,
    pub new_map_id: ActorUniqueID,
}