use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

use crate::types::container_type::ContainerType;

#[gamepacket(id = 47)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ContainerClosePacket {
    container_id: u8,
    container_type: ContainerType,
    server_initiated_close: bool,
}
