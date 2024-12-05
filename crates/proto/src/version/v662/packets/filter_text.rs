use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 163)]
#[derive(ProtoCodec)]
pub struct FilterTextPacket {
    pub text: String,
    pub from_server: bool,
}