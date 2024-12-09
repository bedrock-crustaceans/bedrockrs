use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 40)]
#[derive(ProtoCodec)]
pub struct SetActorMotionPacket {
    pub target_runtime_id: ActorRuntimeID,
    #[endianness(le)]
    pub motion: Vec3<f32>,
    #[endianness(var)]
    pub server_tick: u64,
}