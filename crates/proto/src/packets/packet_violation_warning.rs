use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 156)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct PacketViolationWarningPacket {
    kind: VAR<u32>,
    severity: VAR<u32>,
    violating_packet_id: VAR<u32>,
    context: String,
}
