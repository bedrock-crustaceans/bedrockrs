use bedrockrs_core::Vec2;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 57)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerInputPacket {
    #[endianness(le)]
    pub move_vector: Vec2<f32>,
    pub jumping: bool,
    pub sneaking: bool,
}