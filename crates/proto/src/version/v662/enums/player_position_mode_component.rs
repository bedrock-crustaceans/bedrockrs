pub mod PlayerPositionModeComponent {
    use bedrockrs_macros::ProtoCodec;
    use crate::version::v662::enums::{ActorType, MinecraftEventing};

    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum PositionMode {
        Normal = 0,
        Respawn = 1,
        Teleport {
            #[endianness(le)]
            teleportation_cause: i32,
            #[endianness(le)]
            source_actor_type: i32,
        } = 2,
        OnlyHeadRot = 3,
    }
}