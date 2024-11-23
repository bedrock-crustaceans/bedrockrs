use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct CameraInstruction{
    pub set: Option<()>, // TODO: () --> struct CameraInstruction::SetInstruction
    pub clear: Option<bool>,
    pub fade: Option<()>, // TODO: () --> struct CameraInstruction::FadeInstruction
}