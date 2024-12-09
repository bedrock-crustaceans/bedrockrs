use crate::version::v662::enums::ActorEvent;
use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 27)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ActorEventPacket {
    pub target_runtime_id: ActorRuntimeID,
    pub event_id: ActorEvent,
    #[endianness(var)]
    pub data: i32,
}
