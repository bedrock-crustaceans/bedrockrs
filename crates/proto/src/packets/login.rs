use bedrockrs_core::int::BE;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::connection_request::ConnectionRequest;

#[derive(ProtoCodec, Debug, Clone)]
pub struct LoginPacket {
    pub client_network_version: BE<i32>,
    pub connection_request: ConnectionRequest,
}
