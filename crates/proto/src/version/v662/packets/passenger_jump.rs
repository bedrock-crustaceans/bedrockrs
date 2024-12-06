use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 20)]
#[derive(ProtoCodec)]
pub struct PassengerJumpPacket {
    #[endianness(var)]
    pub jump_scale: i32,
}