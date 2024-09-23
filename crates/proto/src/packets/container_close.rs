use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

use crate::types::container_type::ContainerType;

#[gamepacket(id = 47)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ContainerClosePacket {
    pub container_id: u8,
    pub container_type: ContainerType,
    pub server_initiated_close: bool,
}
