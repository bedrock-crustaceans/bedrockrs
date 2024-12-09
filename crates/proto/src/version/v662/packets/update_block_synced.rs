use crate::version::v662::enums::ActorBlockSyncMessage;
use crate::version::v662::types::NetworkBlockPosition;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 110)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct UpdateBlockSyncedPacket {
    pub block_position: NetworkBlockPosition,
    #[endianness(var)]
    pub block_runtime_id: u32,
    #[endianness(var)]
    pub flags: u32,
    #[endianness(var)]
    pub later: u32,
    #[endianness(var)]
    pub unique_actor_id: i64,
    pub actor_sync_message: ActorBlockSyncMessage::MessageId,
}