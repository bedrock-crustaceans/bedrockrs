use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 130)]
#[derive(ProtoCodec)]
pub struct OnScreenTextureAnimationPacket {
    #[endianness(le)]
    pub effect_id: u32,
}