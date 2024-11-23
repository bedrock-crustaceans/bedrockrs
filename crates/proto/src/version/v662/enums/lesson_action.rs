use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum LessonAction {
    Start = 0,
    Complete = 1,
    Restart = 2,
}