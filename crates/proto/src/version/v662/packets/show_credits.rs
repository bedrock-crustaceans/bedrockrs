use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
enum CreditsState {
    Start = 0,
    Finished = 1,
}

#[gamepacket(id = 75)]
#[derive(ProtoCodec)]
pub struct ShowCreditsPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub credits_state: CreditsState
}