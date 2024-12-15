use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::SpawnPositionType;
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 43)]
#[derive(ProtoCodec)]
pub struct SetSpawnPositionPacket {
    pub spawn_position_type: SpawnPositionType,
    pub block_position: NetworkBlockPosition,
    pub dimension_type: i32,
    pub spawn_block_pos: NetworkBlockPosition,
}