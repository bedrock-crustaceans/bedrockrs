use crate::version::v662::types::MoveActorDeltaData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 111)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct MoveActorDeltaPacket {
    pub move_data: MoveActorDeltaData,
}