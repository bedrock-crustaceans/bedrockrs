use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum LessonAction {
    Start = 0,
    Complete = 1,
    Restart = 2,
}