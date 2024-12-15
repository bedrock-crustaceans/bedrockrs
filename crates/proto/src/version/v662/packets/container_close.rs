use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ContainerID;

#[gamepacket(id = 47)]
#[derive(ProtoCodec)]
pub struct ContainerClosePacket {
    pub container_id: ContainerID,
    pub server_initiated_close: bool,
}