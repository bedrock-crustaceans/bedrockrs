use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{CompoundTag, NetworkBlockPosition};

#[gamepacket(id = 56)]
#[derive(ProtoCodec)]
pub struct BlockActorDataPacket {
    pub block_position: NetworkBlockPosition,
    pub actor_data_tags: CompoundTag,
}