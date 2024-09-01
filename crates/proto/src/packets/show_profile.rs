use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 104)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowProfilePacket {
    xuid: String,
}
