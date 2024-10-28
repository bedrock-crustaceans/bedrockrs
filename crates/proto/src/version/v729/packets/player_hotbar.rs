use crate::version::v729::types::container_id::ContainerID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 48)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerHotbarPacket {
    #[endianness(var)]
    pub selected_slot: u32,
    pub container_id: ContainerID,
    pub should_select_slot: bool,
}
