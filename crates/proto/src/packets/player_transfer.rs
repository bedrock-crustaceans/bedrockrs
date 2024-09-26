use bedrockrs_core::int::LE;
use bedrockrs_macros::{gamepacket, gamepackets, ProtoCodec};

#[gamepacket(id = 85)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct TransferPlayerPacket {
    addr: String,
    port: LE<u16>,
}
