use bedrockrs_core::int::VAR;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 156)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PacketViolationWarningPacket {
    pub kind: VAR<i32>,
    pub severity: VAR<i32>,
    pub violating_packet_id: VAR<i32>,
    pub context: String,
}
