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
            teleportation_cause: MinecraftEventing::TeleportationCause, // TODO: same here. listed as int without an enum.
            source_actor_type: ActorType, // TODO: listed as int without enum. ActorType is assumed, but it is a varint enum.
        } = 2,
        OnlyHeadRot = 3,
    }
}