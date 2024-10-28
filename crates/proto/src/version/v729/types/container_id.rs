use bedrockrs_macros::ProtoCodec;

/// Defines an enum for container IDs, used to identify different types of containers.
#[derive(ProtoCodec, Debug, Copy, Clone, Eq, PartialEq)]
#[enum_repr(i8)]
pub enum ContainerID {
    /// Used to indicate that no container is selected.
    None = -1,
    /// Identifies the player's inventory.
    Inventory = 0,
    /// Indicates the starting container.
    First = 1,
    /// Indicates the ending container.
    Last = 100,
    /// Identifies the player's off-hand inventory slot.
    Offhand = 119,
    /// Identifies the player's armor slots.
    Armor = 120,
    /// Identifies the player's selection slots.
    SelectionSlots = 122,
    /// Identifies the player's dedicated UI interface.
    PlayerOnlyUi = 124,
}
