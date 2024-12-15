use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{ContainerID, ContainerType};
use crate::version::v662::types::{ActorUniqueID, NetworkBlockPosition};

#[gamepacket(id = 46)]
#[derive(ProtoCodec)]
pub struct ContainerOpenPacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    pub position: NetworkBlockPosition,
    pub target_actor_id: ActorUniqueID,
}