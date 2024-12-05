use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CameraInstruction;

#[gamepacket(id = 300)]
#[derive(ProtoCodec)]
pub struct CameraInstructionPacket {
    pub camera_instruction: CameraInstruction,
}