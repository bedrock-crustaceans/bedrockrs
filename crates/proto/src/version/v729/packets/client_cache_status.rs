use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 129)]
#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct ClientCacheStatusPacket {
    pub cache_supported: bool,
}
