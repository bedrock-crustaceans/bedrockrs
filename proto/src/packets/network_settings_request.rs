use bedrock_core::*;
use proto_derive::ProtoCodec;

#[derive(Debug, Copy, Clone, ProtoCodec)]
pub struct NetworkSettingsRequestPacket {
    pub client_network_version: BE::<i32>,
}
