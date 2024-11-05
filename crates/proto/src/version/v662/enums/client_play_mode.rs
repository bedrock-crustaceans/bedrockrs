use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ClientPlayMode {
    Normal = 0,
    Teaser = 1,
    Screen = 2,
    Viewer = 3,
    Reality = 4,
    Placement = 5,
    LivingRoom = 6,
    ExitLevel = 7,
    ExitLevelLivingRoom = 8,
    NumModes = 9,
}