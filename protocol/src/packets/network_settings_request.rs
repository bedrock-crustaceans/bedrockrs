use bedrock_core::types::*;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, Copy, Clone, MCProtoSerialize, MCProtoDeserialize)]
pub struct NetworkSettingsRequestPacket {
    pub client_network_version: i32be,
}
