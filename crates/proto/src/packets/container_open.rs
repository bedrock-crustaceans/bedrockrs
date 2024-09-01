use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

use crate::types::{
    container_id::ContainerID, container_type::ContainerType, network_block_pos::NetworkBlockPos,
};

#[gamepacket(id = 46)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ContainerOpenPacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    pub position: NetworkBlockPos,
    pub target_actor_id: ActorUniqueID,
}
