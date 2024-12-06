use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 86)]
#[derive(ProtoCodec)]
pub struct PlaySoundPacket {
    pub name: String,
    pub position: NetworkBlockPosition,
    #[endianness(le)]
    pub volume: f32,
    #[endianness(le)]
    pub pitch: f32,
}