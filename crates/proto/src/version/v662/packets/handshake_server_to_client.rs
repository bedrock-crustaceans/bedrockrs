use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 3)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct HandshakeServerToClientPacket {
    pub token: String,
}
