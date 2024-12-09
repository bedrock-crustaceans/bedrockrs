use crate::version::v662::types::MoveActorDeltaData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 111)]
#[derive(ProtoCodec)]
pub struct MoveActorDeltaPacket {
    pub move_data: MoveActorDeltaData,
}