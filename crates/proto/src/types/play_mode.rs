#[derive(Debug, Clone)]
pub enum PlayMode {
    Normal,
    Teaser,
    Screen,
    Viewer,
    Reality,
    Placement,
    LivingRoom,
    ExitLevel,
    ExitLevelLivingRoom,
}

impl PlayMode {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}