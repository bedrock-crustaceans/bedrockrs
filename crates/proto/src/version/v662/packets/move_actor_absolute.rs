use crate::version::v662::types::MoveActorAbsoluteData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 18)]
#[derive(ProtoCodec)]
pub struct MoveActorAbsolutePacket {
    pub move_data: MoveActorAbsoluteData
}