use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::EasingType;

#[derive(ProtoCodec, Clone, Debug)]
pub struct EaseData {
    pub ease_type: EasingType,
    #[endianness(le)]
    pub ease_time: f32,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct SetInstruction {
    #[endianness(le)]
    pub runtime_id: i32,
    pub ease_data: Option<EaseData>,
    #[endianness(le)]
    pub position: Option<Vec3<f32>>,
    #[endianness(le)]
    pub rotation: Option<Vec2<f32>>,
    #[endianness(le)]
    pub facing: Option<Vec3<f32>>,
    pub default_preset: Option<bool>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct TimeData {
    #[endianness(le)]
    pub fade_in_time: f32,
    #[endianness(le)]
    pub wait_time: f32,
    #[endianness(le)]
    pub fade_out_time: f32,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct Color {
    #[endianness(le)]
    pub r: f32,
    #[endianness(le)]
    pub g: f32,
    #[endianness(le)]
    pub b: f32,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct FadeInstruction {
    pub time_data: Option<TimeData>,
    pub color: Option<Color>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraInstruction {
    pub set: Option<SetInstruction>,
    pub clear: Option<bool>,
    pub fade: Option<FadeInstruction>,
}

// VERIFY: SetInstruction & FadeInstruction