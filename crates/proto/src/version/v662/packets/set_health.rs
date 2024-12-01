use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 42)]
#[derive(ProtoCodec)]
pub struct SetHealthPacket {
    #[endianness(var)]
    pub health: i32,
}
