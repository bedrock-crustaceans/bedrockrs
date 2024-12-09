use crate::version::v662::types::CameraInstruction;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 300)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraInstructionPacket {
    pub camera_instruction: CameraInstruction,
}