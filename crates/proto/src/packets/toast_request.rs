use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 186)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ToastRequestPacket {
    pub title: String,
    pub content: String,
}
