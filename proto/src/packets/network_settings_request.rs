use bedrockrs_core::*;
use bedrockrs_core::int::BE;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct NetworkSettingsRequestPacket {
    pub client_network_version: BE<i32>,
}
