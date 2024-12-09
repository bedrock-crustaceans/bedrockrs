use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraInstruction{
    pub set: Option<()>, // TODO: CameraInstruction::SetInstruction
    pub clear: Option<bool>,
    pub fade: Option<()>, // TODO: CameraInstruction::FadeInstruction
}