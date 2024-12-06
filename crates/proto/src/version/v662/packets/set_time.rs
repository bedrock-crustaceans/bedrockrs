use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 10)]
#[derive(ProtoCodec)]
pub struct SetTimePacket {
    #[endianness(var)]
    pub time: i32,
}
