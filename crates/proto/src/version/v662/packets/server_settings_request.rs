use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 102)]
#[derive(ProtoCodec)]
pub struct ServerSettingsRequestPacket {}