use bedrock_core::*;
use proto_derive::ProtoCodec;

use crate::types::connection_request::ConnectionRequestType;

#[derive(Debug, ProtoCodec)]
pub struct LoginPacket {
    pub client_network_version: BE<i32>,
    pub connection_request: ConnectionRequestType,
}
