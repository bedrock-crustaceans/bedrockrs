use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 59)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetCommandsEnabledPacket {
    pub enabled: bool,
}
