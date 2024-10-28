use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 85)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct TransferPlayerPacket {
    addr: String,
    #[endianness(le)]
    port: u16,
}
