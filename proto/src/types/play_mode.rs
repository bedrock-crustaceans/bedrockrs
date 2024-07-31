use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::VAR;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum  PlayMode {
    Normal = 0,
    Teaser = 1,
    Screen = 2,
    Viewer = 3,
    Reality = 4,
    Placement = 5,
    LivingRoom = 6,
    ExitLevel = 7,
    ExitLevelLivingRoom = 8,
}
