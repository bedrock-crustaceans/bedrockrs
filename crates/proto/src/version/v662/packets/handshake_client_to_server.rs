use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 4)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct HandshakeClientToServerPacket {}
