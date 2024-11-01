use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ConnectionRequest;

#[gamepacket(id = 1)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct LoginPacket {
    #[endianness(be)]
    pub client_network_version: i32,
    pub connection_request: ConnectionRequest
}
