use bedrockrs_core::*;
use bedrockrs_core::int::BE;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::connection_request::ConnectionRequestType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct LoginPacket {
    pub client_network_version: BE<i32>,
    pub connection_request: ConnectionRequestType,
}
