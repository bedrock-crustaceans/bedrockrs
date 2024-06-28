use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct ClientCacheStatusPacket {
    pub cache_supported: bool,
}
