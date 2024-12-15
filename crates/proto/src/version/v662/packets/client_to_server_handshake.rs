use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 4)]
#[derive(ProtoCodec)]
pub struct ClientToServerHandshakePacket {}