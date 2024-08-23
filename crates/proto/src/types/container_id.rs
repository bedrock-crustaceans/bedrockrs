use bedrockrs_core::int::{BE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

/// 定义了容器ID的枚举，用于标识不同的容器类型。
/// Defines an enum for container IDs, used to identify different types of containers.
#[derive(ProtoCodec, Debug, Copy, Clone, Eq, PartialEq)]
#[enum_repr(BE::<i8>)]
pub enum ContainerID {
    /// 无容器ID，用于表示没有选择任何容器。
    /// No container ID, used to indicate that no container is selected.
    CONTAINER_ID_NONE = -1,
    /// 库存容器ID，标识玩家的物品栏。
    /// Inventory container ID, identifies the player's inventory.
    CONTAINER_ID_INVENTORY = 0,
    /// 第一个容器ID，用于标识开始的容器。
    /// First container ID, used to indicate the starting container.
    CONTAINER_ID_FIRST = 1,
    /// 最后一个容器ID，用于标识结束的容器。
    /// Last container ID, used to indicate the ending container.
    CONTAINER_ID_LAST = 100,
    /// 非主手容器ID，用于标识玩家的副手物品栏。
    /// Off-hand container ID, identifies the player's off-hand inventory slot.
    CONTAINER_ID_OFFHAND = 119,
    /// 防御容器ID，用于标识玩家的防具栏。
    /// Armor container ID, identifies the player's armor slots.
    CONTAINER_ID_ARMOR = 120,
    /// 选择槽容器ID，用于标识玩家的选择槽。
    /// Selection slots container ID, identifies the player's selection slots.
    CONTAINER_ID_SELECTION_SLOTS = 122,
    /// 玩家专属UI容器ID，用于标识玩家的专属UI界面。
    /// Player-only UI container ID, identifies the player's dedicated UI interface.
    CONTAINER_ID_PLAYER_ONLY_UI = 124,
}
