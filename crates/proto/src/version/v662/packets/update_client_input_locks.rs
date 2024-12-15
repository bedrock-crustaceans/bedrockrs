use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::Vec3;

#[gamepacket(id = 196)]
#[derive(ProtoCodec)]
pub struct UpdateClientInputLocksPacket {
    #[endianness(var)]
    pub input_lock_component_data: i32,
    pub server_pos: Vec3,
}