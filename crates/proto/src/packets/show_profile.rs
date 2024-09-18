use xuid::Xuid;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 104)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowProfilePacket {
    pub xuid: Xuid,
}
