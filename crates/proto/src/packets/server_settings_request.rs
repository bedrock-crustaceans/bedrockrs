use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 102)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerSettingsRequestPacket {}
