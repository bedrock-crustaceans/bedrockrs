use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 3)]
#[derive(ProtoCodec)]
pub struct ServerToClientHandshakePacket {
    pub handshake_web_token: String,
}