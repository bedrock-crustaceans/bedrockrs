use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkItemInstanceDescriptor;

#[derive(ProtoCodec)]
struct WriteEntry {
    #[endianness(var)]
    pub creative_net_id: u32,
    pub item_instance: NetworkItemInstanceDescriptor,
}

#[gamepacket(id = 145)]
#[derive(ProtoCodec)]
pub struct CreativeContentPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub write_entries: Vec<WriteEntry>
}