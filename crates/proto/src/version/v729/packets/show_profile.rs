use bedrockrs_macros::{gamepacket, ProtoCodec};
use xuid::Xuid;

#[gamepacket(id = 104)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowProfilePacket {
    pub xuid: Xuid,
}
