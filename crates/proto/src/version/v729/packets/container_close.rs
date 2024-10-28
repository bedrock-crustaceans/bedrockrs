use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::version::v729::types::container_type::ContainerType;

#[gamepacket(id = 47)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ContainerClosePacket {
    pub container_id: u8,
    pub container_type: ContainerType,
    pub server_initiated_close: bool,
}
