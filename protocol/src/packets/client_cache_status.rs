use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, Copy, Clone, MCProtoSerialize, MCProtoDeserialize)]
pub struct ClientCacheStatusPacket {
    pub cache_supported: bool,
}
