use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 140)]
#[derive(ProtoCodec)]
pub struct SettingsCommandPacket {
    pub command: String,
    pub suppress_output: bool,
}