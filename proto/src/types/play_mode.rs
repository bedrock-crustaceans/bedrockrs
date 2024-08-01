use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::Vec3;

#[derive(Debug, Clone)]
pub enum PlayMode {
    Normal,
    Teaser,
    Screen,
    Viewer,
    Reality(Vec3<LE<f32>>),
    Placement,
    LivingRoom,
    ExitLevel,
    ExitLevelLivingRoom,
}
