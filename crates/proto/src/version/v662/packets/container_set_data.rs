use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ContainerID;

#[gamepacket(id = 51)]
#[derive(ProtoCodec)]
pub struct ContainerSetDataPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub id: i32,
    #[endianness(var)]
    pub value: i32,
}