use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 59)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetCommandsEnabledPacket {
    enabled: bool,
}
