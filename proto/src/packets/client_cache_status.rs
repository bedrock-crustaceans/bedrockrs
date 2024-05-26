use proto_derive::ProtoCodec;

#[derive(Debug, Copy, Clone, ProtoCodec)]
pub struct ClientCacheStatusPacket {
    pub cache_supported: bool,
}
