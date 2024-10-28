use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::version::v729::types::connection_request::ConnectionRequest;

#[gamepacket(id = 1)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct LoginPacket {
    #[endianness(be)]
    pub client_network_version: i32,
    pub connection_request: ConnectionRequest,
}
