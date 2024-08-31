use bedrockrs_proto_derive::{gamepacket, gamepackets, ProtoCodec};

#[gamepacket(id = 85)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct TransferPlayerPacket {
    addr: String,
    port: u16,
}
