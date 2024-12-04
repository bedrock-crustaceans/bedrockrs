use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 150)]
#[derive(ProtoCodec)]
pub struct CodeBuilderPacket {
    pub url: String,
    pub should_open_code_builder: bool,
}