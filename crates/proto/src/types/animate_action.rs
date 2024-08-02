#[derive(Debug, Clone)]
pub enum AnimateAction {
    NoAction,
    Swing { rowing_time: f32 },
    WakeUp,
    CriticalHit,
    MagicCriticalHit,
    RowRight,
    RowLeft,
}
