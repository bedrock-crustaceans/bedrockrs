use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 104)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowProfilePacket {
    xuid: String,
}
