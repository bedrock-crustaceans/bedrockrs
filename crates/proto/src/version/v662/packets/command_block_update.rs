use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, NetworkBlockPosition};

#[gamepacket(id = 78)]
#[derive(ProtoCodec)]
pub struct CommandBlockUpdatePacket {
    pub is_block: bool,
    pub target_runtime_id: Option<ActorRuntimeID>, // Only if is_block is false
    pub block_position: Option<NetworkBlockPosition>, // Only if is_block is true
    #[endianness(var)]
    pub command_block_mode: Option<u32>, // Only if is_block is true
    pub redstone_mode: Option<bool>, // Only if is_block is true
    pub is_conditional: Option<bool>, // Only if is_block is true
    pub command: String,
    pub last_output: String,
    pub name: String,
    pub track_output: bool,
    #[endianness(le)]
    pub tick_delay: u32,
    pub should_execute_on_first_tick: bool,
}

// TODO: custom proto impl
