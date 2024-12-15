use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{ContainerID, ContainerType};
use crate::version::v662::types::{ActorUniqueID, CompoundTag};

#[gamepacket(id = 80)]
#[derive(ProtoCodec)]
pub struct UpdateTradePacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    #[endianness(var)]
    pub size: i32,
    #[endianness(var)]
    pub trade_tier: i32,
    pub target_actor_id: ActorUniqueID,
    pub last_trading_player_id: ActorUniqueID,
    pub display_name: String,
    pub use_new_trade_ui: bool,
    pub using_economy_trade: bool,
    pub data_tags: CompoundTag,
}