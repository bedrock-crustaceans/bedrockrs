use bedrockrs_core::Vec3;

#[derive(Debug, Clone)]
pub enum PlayMode {
    Normal,
    Teaser,
    Screen,
    Viewer,
    Reality(Vec3<f32>),
    Placement,
    LivingRoom,
    ExitLevel,
    ExitLevelLivingRoom,
}
