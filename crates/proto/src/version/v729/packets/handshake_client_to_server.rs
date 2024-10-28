use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 4)]
#[derive(Debug, Clone, ProtoCodec)]
pub struct HandshakeClientToServerPacket {}
