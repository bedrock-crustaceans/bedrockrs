use crate::types::container_id::ContainerID;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 48)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerHotbarPacket {
    pub selected_slot: VAR<u32>,
    pub container_id: ContainerID,
    pub should_select_slot: bool,
}
