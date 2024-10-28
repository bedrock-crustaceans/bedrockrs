use bedrockrs_core::Vec3;

#[derive(Debug, Clone)]
pub enum InteractAction {
    Invalid,
    StopRiding(Vec3<f32>),
    InteractUpdate(Vec3<f32>),
    NpcOpen,
    OpenInventory,
}
