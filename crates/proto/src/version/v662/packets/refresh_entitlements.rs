use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 305)]
#[derive(ProtoCodec)]
pub struct RefreshEntitlementsPacket {}