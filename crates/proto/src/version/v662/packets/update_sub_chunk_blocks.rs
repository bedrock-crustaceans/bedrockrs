use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ActorBlockSyncMessage;
use crate::version::v662::types::NetworkBlockPosition;

#[derive(ProtoCodec)]
struct BlocksChangedEntry {
    pub pos: NetworkBlockPosition,
    #[endianness(var)]
    pub runtime_id: u32,
    #[endianness(var)]
    pub update_flags: u32,
    #[endianness(var)]
    pub sync_message_entity_unique_id: u64,
    pub sync_message: ActorBlockSyncMessage::MessageId, // TODO: custom proto, this is sent as unsigned varint, needs to be varint64
}

#[gamepacket(id = 172)]
#[derive(ProtoCodec)]
pub struct UpdateSubChunkBlocksPacket {
    pub sub_chunk_block_position: NetworkBlockPosition,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub standard_blocks_changed: Vec<BlocksChangedEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub extra_blocks_changed: Vec<BlocksChangedEntry>,
}