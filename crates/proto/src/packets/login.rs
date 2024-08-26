use bedrockrs_core::int::BE;
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

use crate::types::connection_request::ConnectionRequest;

#[gamepacket(id = 1)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct LoginPacket {
    pub client_network_version: BE<i32>,
    pub connection_request: ConnectionRequest,
}
