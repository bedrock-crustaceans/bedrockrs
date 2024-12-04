use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 129)]
#[derive(ProtoCodec)]
pub struct ClientCacheStatusPacket {
    pub is_cache_supported: bool,
}