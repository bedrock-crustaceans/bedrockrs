use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, Vec3};

#[gamepacket(id = 40)]
#[derive(ProtoCodec)]
pub struct SetActorMotionPacket {
    pub target_runtime_id: ActorRuntimeID,
    pub motion: Vec3,
    #[endianness(var)]
    pub server_tick: u64,
}