use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 160)]
#[derive(ProtoCodec)]
pub struct PlayerFogPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub fog_stack: Vec<String>
}