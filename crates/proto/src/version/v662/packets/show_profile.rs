use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 104)]
#[derive(ProtoCodec)]
pub struct ShowProfilePacket {
    pub player_xuid: String,
}