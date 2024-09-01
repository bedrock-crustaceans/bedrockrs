use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 10)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetTimePacket {
    pub time: VAR<i32>,
}
