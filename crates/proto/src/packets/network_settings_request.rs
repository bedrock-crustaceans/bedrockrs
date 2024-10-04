use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 193)]
#[derive(ProtoCodec, Debug, Copy, Clone)]
pub struct NetworkSettingsRequestPacket {
    #[endianness(be)]
    pub client_network_version: i32,
}
