use bedrockrs_core::int::LE;
use bedrockrs_proto_derive::{gamepacket, gamepackets, ProtoCodec};

#[gamepacket(id = 85)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct TransferPlayerPacket {
    addr: String,
    port: LE<u16>,
}
