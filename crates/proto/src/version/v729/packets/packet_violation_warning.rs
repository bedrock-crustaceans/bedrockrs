use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 156)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PacketViolationWarningPacket {
    #[endianness(var)]
    pub kind: i32,
    #[endianness(var)]
    pub severity: i32,
    #[endianness(var)]
    pub violating_packet_id: i32,
    pub context: String,
}
